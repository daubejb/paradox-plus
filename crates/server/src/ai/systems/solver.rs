use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::SeedableRng;
use rand::RngCore;
use rand_chacha::ChaCha8Rng;
use bevy::tasks::futures_lite::future;
use fixed::types::I32F32;

use crate::ai::components::{
    ActiveBotTurn, ActiveMdpSolverTask, Bot, Player, Ball, WagerToken, DeterministicRng, StructuralEpoch, CourseTrackResource
};
use crate::ai::mdp_state::MdpState;
use crate::ai::mdp_solver::{
    MdpSolverTable, initialize_table, value_iteration_sweep, get_transitions, calculate_bellman_update
};
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use crate::systems::validation::ClientActionEvent;
use protocol::messages::{ClientAction, CardType};

/// Spawns the off-thread solver task for active bot players.
pub fn trigger_ai_solver_system(
    mut commands: Commands,
    active_bot_query: Query<(Entity, &Player, &Ball, Option<&Bot>), With<ActiveBotTurn>>,
    course: Res<CourseTrackResource>,
    wagers_query: Query<&WagerToken>,
    structural_epoch: Res<StructuralEpoch>,
    mut rng: ResMut<DeterministicRng>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for (entity, player, _ball, _bot) in active_bot_query.iter() {
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag_thread = cancel_flag.clone();

        // Seed a local PRNG using main thread RNG to preserve determinism
        let seed = rng.0.next_u64();
        let _local_rng = ChaCha8Rng::seed_from_u64(seed);

        let course_clone = course.0.clone();
        let active_player_id = player.player_id;
        
        let placed_wagers: Vec<protocol::messages::WagerToken> = wagers_query.iter().map(|w| {
            protocol::messages::WagerToken {
                card_type: w.card_type,
                owner_id: w.owner_id,
                cell_index: w.cell_index as u32,
            }
        }).collect();

        let task = thread_pool.spawn(async move {
            let mut table = MdpSolverTable::new();
            initialize_table(&mut table, &course_clone);
            
            let converged = value_iteration_sweep(
                &mut table,
                &course_clone,
                &placed_wagers,
                active_player_id,
                &cancel_flag_thread,
            );
            
            if converged {
                Some(table)
            } else {
                None
            }
        });

        commands.entity(entity).insert(ActiveMdpSolverTask {
            task: Some(task),
            cancel_flag,
            structural_epoch: structural_epoch.clone(),
        });
        commands.entity(entity).remove::<ActiveBotTurn>();
    }
}

/// Helper to select action based on solved table and bot difficulty.
fn select_action(
    player: &Player,
    ball: &Ball,
    bot: Option<&Bot>,
    table: &MdpSolverTable,
    course: &ActiveCourseTrack,
    wagers_query: &Query<&WagerToken>,
    rng: &mut ResMut<DeterministicRng>,
) -> ClientAction {
    let state = MdpState::new(ball.cell_index, ball.direction, ball.origin_cell, heapless::Vec::new());
    let active_player_id = player.player_id;
    let placed_wagers: Vec<protocol::messages::WagerToken> = wagers_query.iter().map(|w| {
        protocol::messages::WagerToken {
            card_type: w.card_type,
            owner_id: w.owner_id,
            cell_index: w.cell_index as u32,
        }
    }).collect();

    let mut transitions_buf = heapless::Vec::new();

    // 1 die expected value
    get_transitions(state.clone(), 1, course, &placed_wagers, active_player_id, &table.values, &mut transitions_buf);
    let mut ev_1 = I32F32::ZERO;
    for outcome in &transitions_buf {
        let next_idx = outcome.next_state.to_index().unwrap_or(0);
        let next_val = table.values[next_idx];
        ev_1 = ev_1.saturating_add(calculate_bellman_update(outcome.reward, outcome.probability, next_val));
    }

    // Determine if 2 dice are allowed
    let cell = ball.cell_index;
    let terrain = course.cells.get(cell as usize).copied().unwrap_or(TerrainType::Fairway);
    let on_rough = terrain == TerrainType::Rough;
    let has_own_shield = wagers_query.iter().any(|w| {
        w.cell_index == cell && w.card_type == CardType::Shield && w.owner_id == active_player_id
    });
    let can_roll_2_dice = !on_rough || has_own_shield;

    let (optimal, suboptimal) = if can_roll_2_dice {
        get_transitions(state, 2, course, &placed_wagers, active_player_id, &table.values, &mut transitions_buf);
        let mut ev_2 = I32F32::ZERO;
        for outcome in &transitions_buf {
            let next_idx = outcome.next_state.to_index().unwrap_or(0);
            let next_val = table.values[next_idx];
            ev_2 = ev_2.saturating_add(calculate_bellman_update(outcome.reward, outcome.probability, next_val));
        }
        if ev_1 <= ev_2 {
            (1, 2)
        } else {
            (2, 1)
        }
    } else {
        (1, 1)
    };

    // Evaluate difficulty threshold using float-free deterministic RNG
    let rand_val = I32F32::from_bits(rng.0.next_u32() as i64);
    let difficulty_threshold = bot.map(|b| b.difficulty_threshold).unwrap_or_else(|| I32F32::from_num(0.70));
    let dice_count = if rand_val < difficulty_threshold {
        optimal
    } else {
        suboptimal
    };

    ClientAction::RollDice { dice_count }
}

/// Deterministic greedy fallback action.
fn execute_greedy_fallback() -> ClientAction {
    ClientAction::RollDice { dice_count: 1 }
}

/// Polls active solver tasks non-blockingly.
pub fn poll_ai_solver_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Player, &Ball, Option<&Bot>, &mut ActiveMdpSolverTask)>,
    current_epoch: Res<StructuralEpoch>,
    course: Res<CourseTrackResource>,
    wagers_query: Query<&WagerToken>,
    mut action_events: EventWriter<ClientActionEvent>,
    mut rng: ResMut<DeterministicRng>,
) {
    for (entity, player, ball, bot, mut solver_task) in query.iter_mut() {
        // Stale task check (structural epoch changed)
        if solver_task.structural_epoch.epoch_id != current_epoch.epoch_id {
            solver_task.cancel_flag.store(true, Ordering::Relaxed);
            if let Some(task) = solver_task.task.take() {
                task.detach();
            }
            commands.entity(entity).remove::<ActiveMdpSolverTask>();

            // Trigger deterministic greedy fallback
            let action = execute_greedy_fallback();
            action_events.send(ClientActionEvent {
                player_id: player.player_id,
                action,
            });
            continue;
        }

        // Non-blocking poll
        let mut completed = false;
        let mut solver_result = None;

        if let Some(ref mut task) = solver_task.task {
            if let Some(result) = future::block_on(future::poll_once(task)) {
                completed = true;
                solver_result = result;
            }
        }

        if completed {
            solver_task.task.take();
            commands.entity(entity).remove::<ActiveMdpSolverTask>();

            let action = if let Some(table) = solver_result {
                select_action(player, ball, bot, &table, &course.0, &wagers_query, &mut rng)
            } else {
                execute_greedy_fallback()
            };

            action_events.send(ClientActionEvent {
                player_id: player.player_id,
                action,
            });
        }
    }
}
