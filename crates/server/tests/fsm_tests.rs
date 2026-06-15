use bevy::prelude::*;
use std::net::SocketAddr;
use tokio::sync::mpsc::channel;
use protocol::messages::{ClientAction, ServerUpdate, GameStateEnum, MAX_PACKET_SIZE};
use server::systems::fsm::{ServerGameState, fsm_tick_system};
use server::systems::validation::{
    ServerActionReceiver, NetworkSerializationBuffer, ClientActionEvent, ClientActionMessage
};
use server::systems::broadcast::ServerActionSender;

#[test]
fn test_packet_serialization_without_alloc() {
    let mut buffer = NetworkSerializationBuffer::default();
    assert_eq!(buffer.buffer.len(), MAX_PACKET_SIZE);

    let update = ServerUpdate::StateSync {
        sequence: 42,
        game_state: GameStateEnum::Rolling,
        active_player_id: 1,
        current_hole: 1,
        player_positions: heapless::Vec::new(),
        player_directions: heapless::Vec::new(),
        player_scores: heapless::Vec::new(),
        placed_wagers: heapless::Vec::new(),
    };

    // Serialize using the buffer
    let serialized = postcard::to_slice(&update, &mut buffer.buffer).expect("Serialization failed");
    assert!(serialized.len() > 0);
    assert!(serialized.len() <= MAX_PACKET_SIZE);

    // Deserialize and check
    let deserialized: ServerUpdate = postcard::from_bytes(serialized).expect("Deserialization failed");
    if let ServerUpdate::StateSync { sequence, game_state, active_player_id, .. } = deserialized {
        assert_eq!(sequence, 42);
        assert_eq!(game_state, GameStateEnum::Rolling);
        assert_eq!(active_player_id, 1);
    } else {
        panic!("Wrong update type");
    }
}

#[test]
fn test_unauthorized_action_rejection() {
    let mut app = App::new();
    
    // Setup initial state: player 1 is active, state is AwaitingTurn
    let initial_state = ServerGameState {
        state: GameStateEnum::AwaitingTurn,
        active_player_id: 1,
        current_hole: 1,
        sequence: 0,
    };
    app.insert_resource(initial_state);
    
    let (_action_tx, action_rx) = channel::<ClientActionMessage>(10);
    app.insert_resource(ServerActionReceiver { rx: action_rx });
    
    let (broadcast_tx, _broadcast_rx) = channel::<(SocketAddr, Vec<u8>)>(10);
    app.insert_resource(ServerActionSender {
        tx: broadcast_tx,
        clients: vec![],
    });
    
    app.insert_resource(NetworkSerializationBuffer::default());
    app.add_event::<ClientActionEvent>();
    app.add_systems(Update, fsm_tick_system);

    // 1. Send action from player 2 (unauthorized, out of turn)
    app.world_mut().send_event(ClientActionEvent {
        player_id: 2,
        action: ClientAction::RollDice { dice_count: 2 },
    });
    
    app.update();
    
    // Assert that the state sequence did NOT increase and state remains AwaitingTurn
    let state = app.world().resource::<ServerGameState>();
    assert_eq!(state.sequence, 0);
    assert_eq!(state.state, GameStateEnum::AwaitingTurn);

    // 2. Send action from player 1 (authorized)
    app.world_mut().send_event(ClientActionEvent {
        player_id: 1,
        action: ClientAction::RollDice { dice_count: 2 },
    });
    
    app.update();
    
    // Assert that the state transitioned to Rolling and sequence incremented
    let state = app.world().resource::<ServerGameState>();
    assert_eq!(state.sequence, 1);
    assert_eq!(state.state, GameStateEnum::Rolling);
}
