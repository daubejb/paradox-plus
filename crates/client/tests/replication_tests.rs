use bevy::prelude::*;
use protocol::messages::{GameStateEnum, ServerUpdate};
use protocol::physics::MovementDirection;
use client::network::{ClientNetworkPlugin, ClientActionSender, ServerUpdateReceiver};
use client::replication::{ClientReplicationPlugin, ClientGameState, Player, Ball};
use client::presenter::{FixedToFloatPlugin, BallVisualInterpolation};

#[test]
fn test_client_replication_sync() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(ClientNetworkPlugin);
    app.add_plugins(ClientReplicationPlugin);
    app.add_plugins(FixedToFloatPlugin);

    // Setup channel mock
    let (server_tx, client_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (client_tx, _server_rx) = std::sync::mpsc::channel::<Vec<u8>>();

    app.insert_resource(ServerUpdateReceiver {
        receiver: std::sync::Mutex::new(client_rx),
    });
    app.insert_resource(ClientActionSender {
        sender: client_tx,
    });

    // Verify initial state
    assert_eq!(*app.world().resource::<State<ClientGameState>>().get(), ClientGameState::Lobby);

    // Construct a StateSync update
    let mut player_positions = heapless::Vec::new();
    player_positions.push(2).unwrap(); // Player 1 is at cell 2
    let mut player_directions = heapless::Vec::new();
    player_directions.push(MovementDirection::Forward).unwrap();

    let sync_payload = ServerUpdate::StateSync {
        sequence: 10,
        game_state: GameStateEnum::AwaitingTurn,
        active_player_id: 1234,
        current_hole: 1,
        player_positions,
        player_directions,
        player_scores: heapless::Vec::new(),
        placed_wagers: heapless::Vec::new(),
    };

    let mut buf = [0u8; 1024];
    let serialized = postcard::to_slice(&sync_payload, &mut buf).unwrap();
    server_tx.send(serialized.to_vec()).unwrap();

    // Run app updates to consume and apply the state transition
    app.update();
    app.update();

    // Verify states changed
    assert_eq!(*app.world().resource::<State<ClientGameState>>().get(), ClientGameState::AwaitingTurn);

    // Verify entity was spawned
    let mut player_query = app.world_mut().query::<(&Player, &Ball)>();
    let entities: Vec<(&Player, &Ball)> = player_query.iter(app.world()).collect();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].0.player_id, 1234);
    assert_eq!(entities[0].1.cell_index, 2);
}

#[test]
fn test_fixed_to_float_interpolation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(FixedToFloatPlugin);

    // Spawn an entity with Ball, BallVisualInterpolation and Transform
    let entity = app.world_mut().spawn((
        Ball {
            cell_index: 2,
            direction: MovementDirection::Forward,
            origin_cell: 2,
        },
        BallVisualInterpolation { slide_offset: 0.5 },
        Transform::default(),
    )).id();

    // Run update to trigger fixed-to-float presenter system
    app.update();

    // Verify Transform translation matches expected float coordinate
    let transform = app.world().entity(entity).get::<Transform>().unwrap();
    // 2.5 spacing * 2 = 5.0 cell pos
    // 2.5 spacing * 0.5 offset = 1.25 offset
    // 5.0 + 1.25 = 6.25 expected translation.x
    assert_eq!(transform.translation.x, 6.25);
    assert_eq!(transform.translation.y, 0.0);
    assert_eq!(transform.translation.z, 0.0);
}
