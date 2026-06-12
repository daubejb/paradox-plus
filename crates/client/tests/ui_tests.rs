use bevy::prelude::*;
use bevy::window::{Window, WindowResolution};
use client::ui::{
    components::*,
    ClientUiPlugin,
};
use client::network::events::ClientActionRequest;
use protocol::messages::{ClientAction, ServerUpdate};

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
        bevy::input::InputPlugin,
        AssetPlugin::default(),
        client::network::ClientNetworkPlugin,
        client::replication::ClientReplicationPlugin,
        ClientUiPlugin,
    ));

    app.init_asset::<Image>();
    app.add_event::<ReceivedCharacter>();

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

    // Transition state to SoloSetup, then trigger PlayGameButton click to start game and spawn board
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<ClientScreenState>>();
        next_state.set(ClientScreenState::SoloSetup);
    }
    app.update();
    app.update();

    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<PlayGameButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Play Game button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }
    app.update();
    app.update();
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

    // Verify that the SelectedWagerCard resource has been updated to Some(1)
    {
        let selected_card = app.world().resource::<SelectedWagerCard>();
        assert_eq!(selected_card.0, Some(1), "Expected SelectedWagerCard to be Some(1)");
    }

    // Query for a BoardCellNode with index = 10
    let mut cell_query = app.world_mut().query_filtered::<(Entity, &BoardCellNode), With<Button>>();
    let (cell_entity, _) = cell_query
        .iter(app.world())
        .find(|(_, node)| node.index == 10)
        .expect("Board cell button with index 10 not found");

    // Simulate clicking the board cell
    app.world_mut().entity_mut(cell_entity).insert(Interaction::Pressed);

    // Update to trigger cell click handler system
    app.update();

    // Verify that the SelectedWagerCard resource has been reset to None
    {
        let selected_card = app.world().resource::<SelectedWagerCard>();
        assert_eq!(selected_card.0, None, "Expected SelectedWagerCard to be reset to None");
    }

    // Verify that ClientActionRequest event for DraftCard was dispatched with cell_index = 10
    let events = app.world().resource::<Events<ClientActionRequest>>();
    let mut reader = events.get_reader();
    let sent_events: Vec<&ClientActionRequest> = reader.read(events).collect();

    let draft_card_event = sent_events.iter().find(|event| matches!(event.0, ClientAction::DraftCard { .. }));
    assert!(draft_card_event.is_some(), "Expected a ClientAction::DraftCard event to be sent");
    if let Some(ClientActionRequest(ClientAction::DraftCard { card_type, cell_index })) = draft_card_event {
        assert_eq!(*card_type, 1, "Expected card_type to be 1 (Banana)");
        assert_eq!(*cell_index, 10, "Expected cell_index to match drafted spot (10)");
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

    // Run startup systems
    app.update();

    // Transition state to SoloSetup, then trigger PlayGameButton click
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<ClientScreenState>>();
        next_state.set(ClientScreenState::SoloSetup);
    }
    app.update();
    app.update();

    // Trigger Play Game button click to start match
    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<PlayGameButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Play Game button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }
    // Update to process button click, dispatch StartPractice, and process transitions
    app.update();
    app.update();
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

#[test]
fn test_screen_state_transitions() {
    let mut app = setup_headless_ui_app();

    // Run startup systems
    app.update();

    // Verify initial state is Landing
    {
        let screen_state = app.world().resource::<State<ClientScreenState>>();
        assert_eq!(*screen_state.get(), ClientScreenState::Landing);
    }

    // Verify style visibility in Landing state
    {
        let mut style_query = app.world_mut().query_filtered::<&Style, With<LandingScreenNode>>();
        let landing_style = style_query.get_single(app.world()).expect("Landing screen node missing");
        assert_eq!(landing_style.display, Display::Flex);

        let mut setup_query = app.world_mut().query_filtered::<&Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>();
        let setup_style = setup_query.get_single(app.world()).expect("Setup screen node missing");
        assert_eq!(setup_style.display, Display::None);

        let mut gameplay_query = app.world_mut().query_filtered::<&Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>();
        let gameplay_style = gameplay_query.get_single(app.world()).expect("Gameplay screen node missing");
        assert_eq!(gameplay_style.display, Display::None);
    }

    // Simulate clicking the SoloPractice button to transition to SoloSetup
    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<SoloPracticeButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Solo practice button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }

    // Run update twice to process the button click and apply state transition to SoloSetup
    app.update();
    app.update();

    // Verify state transitioned to SoloSetup
    {
        let screen_state = app.world().resource::<State<ClientScreenState>>();
        assert_eq!(*screen_state.get(), ClientScreenState::SoloSetup);
    }

    // Verify style visibility in SoloSetup state
    {
        let mut style_query = app.world_mut().query_filtered::<&Style, With<LandingScreenNode>>();
        let landing_style = style_query.get_single(app.world()).expect("Landing screen node missing");
        assert_eq!(landing_style.display, Display::None);

        let mut setup_query = app.world_mut().query_filtered::<&Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>();
        let setup_style = setup_query.get_single(app.world()).expect("Setup screen node missing");
        assert_eq!(setup_style.display, Display::Flex);

        let mut gameplay_query = app.world_mut().query_filtered::<&Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>();
        let gameplay_style = gameplay_query.get_single(app.world()).expect("Gameplay screen node missing");
        assert_eq!(gameplay_style.display, Display::None);
    }

    // Simulate clicking the Play Game button to transition to Gameplay
    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<PlayGameButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Play Game button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }

    // Run updates to process play button click, server sync, and transition to Gameplay
    app.update();
    app.update();
    app.update();

    // Verify state transitioned to Gameplay
    {
        let screen_state = app.world().resource::<State<ClientScreenState>>();
        assert_eq!(*screen_state.get(), ClientScreenState::Gameplay);
    }

    // Verify style visibility in Gameplay state
    {
        let mut style_query = app.world_mut().query_filtered::<&Style, With<LandingScreenNode>>();
        let landing_style = style_query.get_single(app.world()).expect("Landing screen node missing");
        assert_eq!(landing_style.display, Display::None);

        let mut setup_query = app.world_mut().query_filtered::<&Style, (With<SoloSetupScreenNode>, Without<LandingScreenNode>)>();
        let setup_style = setup_query.get_single(app.world()).expect("Setup screen node missing");
        assert_eq!(setup_style.display, Display::None);

        let mut gameplay_query = app.world_mut().query_filtered::<&Style, (With<GameplayScreenNode>, Without<LandingScreenNode>, Without<SoloSetupScreenNode>)>();
        let gameplay_style = gameplay_query.get_single(app.world()).expect("Gameplay screen node missing");
        assert_eq!(gameplay_style.display, Display::Flex);
    }

    // Simulate clicking the Hamburger button to return to Landing
    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<HamburgerButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Hamburger button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }

    // Run updates to process button clicks and state exit/enter transitions
    app.update();
    app.update();
    app.update();

    // Verify state transitioned back to Landing
    {
        let screen_state = app.world().resource::<State<ClientScreenState>>();
        assert_eq!(*screen_state.get(), ClientScreenState::Landing);
    }
}

#[test]
fn test_wager_card_qty_hud_rendering() {
    let mut app = setup_headless_ui_app();

    // Run startup systems to spawn HUD
    app.update();

    // Transition to Gameplay state
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<ClientScreenState>>();
        next_state.set(ClientScreenState::Gameplay);
    }
    app.update();

    // Send a mock StateSync event with 1 Shield and 2 Golden Die cards in player's hand
    let mut hand = heapless::Vec::new();
    hand.push(0).unwrap(); // Shield
    hand.push(2).unwrap(); // Golden Die
    hand.push(2).unwrap(); // Golden Die

    let mut scores = heapless::Vec::new();
    scores.push(protocol::messages::Scorecard {
        running_strokes: 3,
        total_strokes: 3,
        earned_cards: hand,
    }).unwrap();

    let sync_event = ServerUpdate::StateSync {
        sequence: 1,
        game_state: protocol::messages::GameStateEnum::AwaitingTurn,
        active_player_id: 1234,
        current_hole: 1,
        player_positions: {
            let mut v = heapless::Vec::new();
            v.push(0).unwrap();
            v
        },
        player_scores: scores,
        placed_wagers: heapless::Vec::new(),
    };

    app.world_mut().resource_mut::<Events<client::network::ServerUpdateEvent>>().send(client::network::ServerUpdateEvent(sync_event));

    // Update to process events and run render system
    app.update();

    // Query WagerCardQtyTextNode text values
    let mut text_query = app.world_mut().query::<(&Text, &WagerCardQtyTextNode)>();
    let mut found_shield = false;
    let mut found_banana = false;
    let mut found_golden = false;

    for (text, node) in text_query.iter(app.world()) {
        let val = &text.sections[0].value;
        match node.card_type {
            0 => {
                assert_eq!(val, "SHIELD (1)");
                found_shield = true;
            }
            1 => {
                assert_eq!(val, "BANANA (0)");
                found_banana = true;
            }
            2 => {
                assert_eq!(val, "GOLDEN (2)");
                found_golden = true;
            }
            _ => {}
        }
    }

    assert!(found_shield);
    assert!(found_banana);
    assert!(found_golden);
}

#[test]
fn test_leaderboard_ticker_hierarchy_and_updates() {
    use protocol::messages::{ServerUpdate, GameStateEnum, Scorecard};
    use client::network::ServerUpdateEvent;

    let mut app = setup_headless_ui_app();
    app.update();

    // Verify presence of Ticker Container and Ticker Track
    let mut container_query = app.world_mut().query_filtered::<Entity, With<LeaderboardTickerContainerNode>>();
    assert!(container_query.get_single(app.world()).is_ok(), "Leaderboard Ticker Container not spawned");

    let mut track_query = app.world_mut().query_filtered::<Entity, With<LeaderboardTickerTrackNode>>();
    assert!(track_query.get_single(app.world()).is_ok(), "Leaderboard Ticker Track not spawned");

    // Send a mock StateSync to trigger update system
    let sync_update = ServerUpdate::StateSync {
        sequence: 1,
        game_state: GameStateEnum::AwaitingTurn,
        active_player_id: 1234,
        current_hole: 1,
        player_positions: {
            let mut v = heapless::Vec::new();
            v.push(10).unwrap();
            v.push(8).unwrap();
            v
        },
        player_scores: {
            let mut v = heapless::Vec::new();
            v.push(Scorecard {
                running_strokes: 3,
                total_strokes: 3,
                earned_cards: heapless::Vec::new(),
            }).unwrap();
            v.push(Scorecard {
                running_strokes: 5,
                total_strokes: 5,
                earned_cards: heapless::Vec::new(),
            }).unwrap();
            v
        },
        placed_wagers: heapless::Vec::new(),
    };

    app.world_mut().send_event(ServerUpdateEvent(sync_update));
    app.update();

    // Check that children were spawned inside the track node
    let track_entity = track_query.get_single(app.world()).unwrap();
    let children = app.world().get::<Children>(track_entity).expect("Track should have spawned children");
    assert_eq!(children.len(), 2, "Expected 2 player pill items in track");
}

