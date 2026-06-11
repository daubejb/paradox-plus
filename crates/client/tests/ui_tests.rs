use bevy::prelude::*;
use bevy::window::{Window, WindowResolution};
use client::ui::{
    components::*,
    ClientUiPlugin,
};
use client::network::events::ClientActionRequest;
use protocol::messages::ClientAction;

fn setup_headless_ui_app() -> App {
    let mut app = App::new();

    // Setup loopback mock channels for tests to prevent panics
    let (action_tx, action_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (update_tx, update_rx) = std::sync::mpsc::channel::<Vec<u8>>();

    app.insert_resource(client::network::ServerUpdateReceiver {
        receiver: std::sync::Mutex::new(update_rx),
    });
    app.insert_resource(client::network::ClientActionSender {
        sender: action_tx,
    });
    app.insert_resource(client::ui::systems::simulation::LocalServerChannels {
        action_rx: std::sync::Mutex::new(action_rx),
        update_tx,
        send_buf: std::sync::Mutex::new(Vec::with_capacity(65536)),
    });

    app.add_plugins((
        MinimalPlugins,
        bevy::state::app::StatesPlugin,
        AssetPlugin::default(),
        client::network::ClientNetworkPlugin,
        client::replication::ClientReplicationPlugin,
        ClientUiPlugin,
    ));

    // Spawn a dummy window to ensure layout computations and updates run
    app.world_mut().spawn(Window {
        resolution: WindowResolution::new(1920.0, 1080.0),
        ..default()
    });

    app
}

#[test]
fn test_ui_node_hierarchy() {
    let mut app = setup_headless_ui_app();

    // Run startup systems to spawn the UI layout
    app.update();

    // Verify RootUiNode is present
    let mut root_query = app.world_mut().query_filtered::<Entity, With<RootUiNode>>();
    let root_entity = root_query.get_single(app.world()).expect("Root UI Node missing");

    // Verify presence of main sections: Top HUD, Central Board, Bottom controls
    let mut top_hud_query = app.world_mut().query_filtered::<Entity, With<TopHudNode>>();
    assert!(top_hud_query.get_single(app.world()).is_ok(), "Top HUD not spawned");

    let mut board_query = app.world_mut().query_filtered::<Entity, With<BoardContainerNode>>();
    assert!(board_query.get_single(app.world()).is_ok(), "Board container not spawned");

    let mut bottom_bar_query = app.world_mut().query_filtered::<Entity, With<BottomBarNode>>();
    assert!(bottom_bar_query.get_single(app.world()).is_ok(), "Bottom Bar not spawned");

    // Verify hierarchy children logic
    let children = app.world().get::<Children>(root_entity).expect("Root has no children");
    assert!(!children.is_empty(), "Root Node has no spawned children");
}

#[test]
fn test_wager_card_selection_interaction() {
    let mut app = setup_headless_ui_app();

    // Run startup systems
    app.update();

    // Query for a WagerCardButtonNode with card_type = 1 (Banana)
    let mut banana_query = app.world_mut().query_filtered::<(Entity, &WagerCardButtonNode), With<Button>>();
    let (banana_entity, _) = banana_query
        .iter(app.world())
        .find(|(_, node)| node.card_type == 1)
        .expect("Banana wager card button not found");

    // Simulate clicking the Banana button
    app.world_mut().entity_mut(banana_entity).insert(Interaction::Pressed);

    // Update to trigger interaction systems
    app.update();

    // Verify that ClientActionRequest event was dispatched
    let events = app.world().resource::<Events<ClientActionRequest>>();
    let mut reader = events.get_reader();
    let sent_events: Vec<&ClientActionRequest> = reader.read(events).collect();

    assert_eq!(sent_events.len(), 1, "Expected exactly one ClientActionRequest to be sent");
    if let ClientAction::DraftCard { card_type, cell_index } = &sent_events[0].0 {
        assert_eq!(*card_type, 1, "Expected card_type to be 1 (Banana)");
        assert_eq!(*cell_index, 10, "Expected cell_index to match drafted spot");
    } else {
        panic!("Sent event was not a ClientAction::DraftCard variant");
    }
}

#[test]
fn test_loopback_payloads_serialization_compliance() {
    use postcard::{to_slice, from_bytes};
    use protocol::messages::{ClientAction, ServerUpdate, GameStateEnum, Scorecard};
    use heapless::Vec as HVec;

    // Test ClientAction
    let action = ClientAction::RollDice { dice_count: 2 };
    let mut buf1 = [0u8; 1024];
    let serialized_action = to_slice(&action, &mut buf1).expect("Failed to serialize ClientAction");
    let deserialized_action: ClientAction = from_bytes(serialized_action).expect("Failed to deserialize ClientAction");
    assert_eq!(action, deserialized_action);

    // Test ServerUpdate
    let mut player_positions = HVec::new();
    player_positions.push(10).unwrap();
    let mut player_scores = HVec::new();
    player_scores.push(Scorecard {
        running_strokes: 3,
        total_strokes: 3,
        earned_cards: HVec::new(),
    }).unwrap();

    let update = ServerUpdate::StateSync {
        sequence: 123,
        game_state: GameStateEnum::AwaitingTurn,
        active_player_id: 999,
        current_hole: 1,
        player_positions,
        player_scores,
        placed_wagers: HVec::new(),
    };

    let mut buf2 = [0u8; 1024];
    let serialized_update = to_slice(&update, &mut buf2).expect("Failed to serialize ServerUpdate");
    let deserialized_update: ServerUpdate = from_bytes(serialized_update).expect("Failed to deserialize ServerUpdate");
    assert_eq!(update, deserialized_update);
}

#[test]
fn test_board_rebuild_on_hole_change() {
    let mut app = setup_headless_ui_app();

    // Verify CurrentHole default starts at u8::MAX sentinel
    {
        let current_hole = app.world().resource::<CurrentHole>();
        assert_eq!(current_hole.0, u8::MAX);
    }

    // Run startup systems (trigger_initial_state_sync replicates current_hole: 1)
    app.update();

    // Verify CurrentHole is now 1
    {
        let current_hole = app.world().resource::<CurrentHole>();
        assert_eq!(current_hole.0, 1);
    }

    // Verify Hole 1 has 27 cells spawned (TeeBox + 26 preset spaces)
    {
        let mut cell_query = app.world_mut().query::<&BoardCellNode>();
        let cell_count = cell_query.iter(app.world()).count();
        assert_eq!(cell_count, 27, "Expected 27 cells spawned for Hole 1");
    }

    // Manually change CurrentHole to 2 and trigger change detection
    {
        let mut current_hole = app.world_mut().resource_mut::<CurrentHole>();
        current_hole.0 = 2;
    }

    // Run systems to trigger rebuilding
    app.update();

    // Verify Hole 2 has 14 cells spawned (TeeBox + 13 preset spaces)
    {
        let mut cell_query = app.world_mut().query::<&BoardCellNode>();
        let cell_count = cell_query.iter(app.world()).count();
        assert_eq!(cell_count, 14, "Expected 14 cells spawned for Hole 2");
    }
}
