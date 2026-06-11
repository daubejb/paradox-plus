use bevy::prelude::*;
use crate::ai::components::{TurnTimer, Player, ActiveBotTurn, ActiveMdpSolverTask};
use crate::systems::fsm::ServerGameState;
use protocol::messages::GameStateEnum;
use fixed::types::I32F32;

/// Monitors turn durations and executes AI takeovers if timeout is reached.
pub fn turn_timeout_takeover_system(
    mut commands: Commands,
    game_state: Res<ServerGameState>,
    time: Res<Time>,
    mut timer: ResMut<TurnTimer>,
    players_query: Query<(Entity, &Player), (Without<ActiveMdpSolverTask>, Without<ActiveBotTurn>)>,
) {
    if game_state.state != GameStateEnum::AwaitingTurn {
        timer.elapsed = I32F32::ZERO;
        return;
    }

    let mut active_entity = None;
    for (entity, player) in players_query.iter() {
        if player.player_id == game_state.active_player_id {
            active_entity = Some(entity);
            break;
        }
    }

    if let Some(entity) = active_entity {
        let delta = I32F32::from_num(time.delta_seconds());
        timer.elapsed = timer.elapsed.saturating_add(delta);

        // 15 seconds turn timeout limit
        if timer.elapsed >= I32F32::from_num(15) {
            commands.entity(entity).insert(ActiveBotTurn);
            timer.elapsed = I32F32::ZERO;
        }
    } else {
        timer.elapsed = I32F32::ZERO;
    }
}
