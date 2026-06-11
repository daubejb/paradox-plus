use bevy::prelude::*;
use protocol::terrain::{ActiveCourseTrack, TerrainType};
use protocol::physics::MovementDirection;
use protocol::messages::{ClientAction, GameStateEnum};
use server::ai::{
    AiPlugin, StructuralEpoch, CourseTrackResource, Bot, ActiveBotTurn, ActiveMdpSolverTask, TurnTimer, Player, Ball
};
use server::systems::fsm::ServerGameState;
use server::systems::validation::ClientActionEvent;

fn create_mock_course() -> ActiveCourseTrack {
    let mut cells = heapless::Vec::new();
    cells.push(TerrainType::TeeBox).unwrap();
    for _ in 1..=5 {
        cells.push(TerrainType::Fairway).unwrap();
    }
    cells.push(TerrainType::Green(0)).unwrap();

    ActiveCourseTrack {
        hole_index: 1,
        par: 3,
        total_cells: 7,
        cells,
    }
}

fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AiPlugin);
    app.add_event::<ClientActionEvent>();
    app.init_resource::<CourseTrackResource>();
    app.init_resource::<ServerGameState>();
    app
}

#[test]
fn test_non_blocking_polling_loop() {
    let mut app = setup_test_app();

    // Insert CourseTrackResource
    let course = create_mock_course();
    app.insert_resource(CourseTrackResource(course));

    // Spawn a bot entity
    let bot_entity = app.world_mut().spawn((
        Player { player_id: 42 },
        Ball {
            cell_index: 1,
            direction: MovementDirection::Forward,
            origin_cell: 1,
        },
        Bot {
            difficulty_threshold: fixed::types::I32F32::from_num(0.8),
        },
        ActiveBotTurn,
    )).id();

    // First update to trigger the solver
    app.update();

    // Verify ActiveBotTurn is removed and ActiveMdpSolverTask is inserted
    assert!(app.world().entity(bot_entity).get::<ActiveBotTurn>().is_none());
    assert!(app.world().entity(bot_entity).get::<ActiveMdpSolverTask>().is_some());

    // Loop app.update() until the task completes (non-blocking poll)
    let mut resolved = false;
    for _ in 0..100 {
        app.update();
        let events = app.world().resource::<Events<ClientActionEvent>>();
        let mut reader = events.get_reader();
        for ev in reader.read(events) {
            if ev.player_id == 42 {
                if let ClientAction::RollDice { dice_count } = ev.action {
                    assert!(dice_count == 1 || dice_count == 2);
                    resolved = true;
                }
            }
        }
        if resolved {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    assert!(resolved, "Bot solver task did not complete or send event");
    // Verify ActiveMdpSolverTask is removed after completion
    assert!(app.world().entity(bot_entity).get::<ActiveMdpSolverTask>().is_none());
}

#[test]
fn test_stale_task_cancellation() {
    let mut app = setup_test_app();

    let course = create_mock_course();
    app.insert_resource(CourseTrackResource(course));

    let bot_entity = app.world_mut().spawn((
        Player { player_id: 42 },
        Ball {
            cell_index: 1,
            direction: MovementDirection::Forward,
            origin_cell: 1,
        },
        Bot {
            difficulty_threshold: fixed::types::I32F32::from_num(0.8),
        },
        ActiveBotTurn,
    )).id();

    // Trigger solver
    app.update();

    assert!(app.world().entity(bot_entity).get::<ActiveMdpSolverTask>().is_some());

    // Mutate the epoch in the resource to make it stale
    let mut epoch = app.world_mut().resource_mut::<StructuralEpoch>();
    epoch.epoch_id += 1;

    // Next update should detect stale task, cancel/detach it, and send fallback
    app.update();

    // Verify ActiveMdpSolverTask is removed
    assert!(app.world().entity(bot_entity).get::<ActiveMdpSolverTask>().is_none());

    // Verify that the greedy fallback RollDice { dice_count: 1 } was dispatched
    let events = app.world().resource::<Events<ClientActionEvent>>();
    let mut reader = events.get_reader();
    let mut found_fallback = false;
    for ev in reader.read(events) {
        if ev.player_id == 42 {
            if let ClientAction::RollDice { dice_count } = ev.action {
                assert_eq!(dice_count, 1, "Stale task fallback must roll 1 die");
                found_fallback = true;
            }
        }
    }
    assert!(found_fallback, "Stale task did not trigger fallback event");
}

#[test]
fn test_turn_timeout_takeover() {
    let mut app = setup_test_app();

    // Setup initial state: player 99 is active, state is AwaitingTurn
    let initial_state = ServerGameState {
        state: GameStateEnum::AwaitingTurn,
        active_player_id: 99,
        current_hole: 1,
        sequence: 0,
    };
    app.insert_resource(initial_state);

    // Spawn the human player
    let player_entity = app.world_mut().spawn((
        Player { player_id: 99 },
    )).id();

    // Run first update to initialize systems and time
    app.update();

    // Verify initial state: TurnTimer is 0, player does not have ActiveBotTurn
    {
        let timer = app.world().resource::<TurnTimer>();
        assert_eq!(timer.elapsed, fixed::types::I32F32::ZERO);
        assert!(app.world().entity(player_entity).get::<ActiveBotTurn>().is_none());
    }

    // Configure manual time update strategy to progress time by 250ms per frame
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(250)));

    // Run app update 64 times to simulate 16 seconds passing (0.25s * 64 = 16s)
    for _ in 0..64 {
        app.update();
    }

    // Verify player now has ActiveBotTurn and TurnTimer is reset to 0
    assert!(app.world().entity(player_entity).get::<ActiveBotTurn>().is_some());
    let timer = app.world().resource::<TurnTimer>();
    assert_eq!(timer.elapsed, fixed::types::I32F32::ZERO);
}
