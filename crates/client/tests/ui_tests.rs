use bevy::prelude::*;
use client::ui::{
    components::*,
    ClientUiPlugin,
};
use client::network::events::ClientActionRequest;
use protocol::messages::{ClientAction, ServerUpdate, CardType};

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
        bevy::window::WindowPlugin::default(),
        bevy::state::app::StatesPlugin,
        bevy::input::InputPlugin,
        TransformPlugin,
        HierarchyPlugin,
        AssetPlugin::default(),
        client::network::ClientNetworkPlugin,
        client::replication::ClientReplicationPlugin,
        ClientUiPlugin,
    ));

    app.init_asset::<Image>();
    app.init_asset::<Mesh>();
    app.init_asset::<ColorMaterial>();




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

    // Populate ClientScorecards with a Banana card in hand (card value = 1)
    {
        use heapless::Vec as HVec;
        let mut hand = HVec::new();
        hand.push(1).unwrap(); // Banana card (card_type = 1)
        app.insert_resource(client::ui::components::ClientScorecards(vec![
            protocol::messages::Scorecard {
                running_strokes: 0,
                total_strokes: 0,
                earned_cards: hand,
                cards_earned_this_hole: HVec::new(),
                strokes_per_hole: HVec::new(),
            }
        ]));
    }

    // Query for a WagerCardButtonNode with card_type = CardType::Banana
    let mut banana_query = app.world_mut().query_filtered::<(Entity, &WagerCardButtonNode), With<Button>>();
    let (banana_entity, _) = banana_query
        .iter(app.world())
        .find(|(_, node)| node.card_type == CardType::Banana)
        .expect("Banana wager card button not found");

    // Simulate clicking the Banana button
    app.world_mut().entity_mut(banana_entity).insert(Interaction::Pressed);

    // Update to trigger interaction systems
    app.update();

    // Verify that the SelectedWagerCard resource has been updated to Some(CardType::Banana)
    {
        let selected_card = app.world().resource::<SelectedWagerCard>();
        assert_eq!(selected_card.0, Some(CardType::Banana), "Expected SelectedWagerCard to be Some(CardType::Banana)");
    }

    // Query for a BoardCellNode with index = 10 and get its position
    let cell_pos = {
        let mut cell_query = app.world_mut().query::<(&BoardCellNode, &Transform)>();
        let (_, cell_transform) = cell_query
            .iter(app.world())
            .find(|(node, _)| node.index == 10)
            .expect("Board cell with index 10 not found");
        cell_transform.translation.xy()
    };

    // Set cursor position override resource
    app.insert_resource(client::ui::components::CursorPositionOverride(Some(cell_pos)));

    // Simulate clicking the board cell using the mouse input resource
    {
        let mut mouse_input = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
        mouse_input.press(MouseButton::Left);
    }

    // Run the clicks handler system directly to prevent PreUpdate clearing the input
    use bevy::ecs::system::RunSystemOnce;
    use client::ui::systems::simulation::board::interaction::handle_board_clicks_system;
    app.world_mut().run_system_once(handle_board_clicks_system);

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
        assert_eq!(*card_type, CardType::Banana, "Expected card_type to be CardType::Banana");
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
        cards_earned_this_hole: HVec::new(),
        strokes_per_hole: HVec::new(),
    }).unwrap();

    let mut player_directions = HVec::new();
    player_directions.push(protocol::physics::MovementDirection::Forward).unwrap();

    let update = ServerUpdate::StateSync {
        sequence: 123,
        game_state: GameStateEnum::AwaitingTurn,
        active_player_id: 999,
        current_hole: 1,
        player_positions,
        player_directions,
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

    // Manually change CurrentHole to 18 and trigger change detection
    {
        let mut current_hole = app.world_mut().resource_mut::<CurrentHole>();
        current_hole.0 = 18;
    }

    // Run systems to trigger rebuilding
    app.update();

    // Verify Hole 18 has 14 cells spawned (TeeBox + 13 preset spaces)
    {
        let mut cell_query = app.world_mut().query::<&BoardCellNode>();
        let cell_count = cell_query.iter(app.world()).count();
        assert_eq!(cell_count, 14, "Expected 14 cells spawned for Hole 18");
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
        cards_earned_this_hole: heapless::Vec::new(),
        strokes_per_hole: heapless::Vec::new(),
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
        player_directions: {
            let mut v = heapless::Vec::new();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
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
            CardType::Shield => {
                assert_eq!(val, "SHIELD (1)");
                found_shield = true;
            }
            CardType::Banana => {
                assert_eq!(val, "BANANA (0)");
                found_banana = true;
            }
            CardType::GoldenDie => {
                assert_eq!(val, "GOLDEN (2)");
                found_golden = true;
            }
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
    use client::ui::components::LeaderboardCompletedHolesScore;

    let mut app = setup_headless_ui_app();
    app.update();

    // Verify presence of Ticker Container and Ticker Track
    let mut container_query = app.world_mut().query_filtered::<Entity, With<LeaderboardTickerContainerNode>>();
    assert!(container_query.get_single(app.world()).is_ok(), "Leaderboard Ticker Container not spawned");

    let mut track_query = app.world_mut().query_filtered::<Entity, With<LeaderboardTickerTrackNode>>();
    assert!(track_query.get_single(app.world()).is_ok(), "Leaderboard Ticker Track not spawned");

    // 1. Send Hole 1 AwaitingTurn: Ticker should reset to Even Par "E" (relative score 0) for both players.
    let sync_update_h1 = ServerUpdate::StateSync {
        sequence: 1,
        game_state: GameStateEnum::AwaitingTurn,
        active_player_id: 1234,
        current_hole: 1,
        player_positions: {
            let mut v = heapless::Vec::new();
            v.push(0).unwrap();
            v.push(0).unwrap();
            v
        },
        player_directions: {
            let mut v = heapless::Vec::new();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v
        },
        player_scores: {
            let mut v = heapless::Vec::new();
            v.push(Scorecard {
                running_strokes: 3,
                total_strokes: 3,
                earned_cards: heapless::Vec::new(),
                cards_earned_this_hole: heapless::Vec::new(),
                strokes_per_hole: heapless::Vec::new(),
            }).unwrap();
            v.push(Scorecard {
                running_strokes: 5,
                total_strokes: 5,
                earned_cards: heapless::Vec::new(),
                cards_earned_this_hole: heapless::Vec::new(),
                strokes_per_hole: heapless::Vec::new(),
            }).unwrap();
            v
        },
        placed_wagers: heapless::Vec::new(),
    };

    app.world_mut().send_event(ServerUpdateEvent(sync_update_h1));
    app.update();

    {
        let completed_scores = app.world().resource::<LeaderboardCompletedHolesScore>();
        assert_eq!(completed_scores.player_par_scores, vec![0, 0], "Hole 1 running strokes should not affect par scores yet");
    }

    // 2. Send Hole 1 Completed: Ticker should update par scores relative to Hole 1's par (par is 6).
    let sync_update_h1_completed = ServerUpdate::StateSync {
        sequence: 2,
        game_state: GameStateEnum::HoleCompleted,
        active_player_id: 1234,
        current_hole: 1,
        player_positions: {
            let mut v = heapless::Vec::new();
            v.push(26).unwrap();
            v.push(26).unwrap();
            v
        },
        player_directions: {
            let mut v = heapless::Vec::new();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v
        },
        player_scores: {
            let mut v = heapless::Vec::new();
            v.push(Scorecard {
                running_strokes: 7, // 7 strokes: +1 over par
                total_strokes: 7,
                earned_cards: heapless::Vec::new(),
                cards_earned_this_hole: heapless::Vec::new(),
                strokes_per_hole: heapless::Vec::new(),
            }).unwrap();
            v.push(Scorecard {
                running_strokes: 5, // 5 strokes: -1 under par
                total_strokes: 5,
                earned_cards: heapless::Vec::new(),
                cards_earned_this_hole: heapless::Vec::new(),
                strokes_per_hole: heapless::Vec::new(),
            }).unwrap();
            v
        },
        placed_wagers: heapless::Vec::new(),
    };

    app.world_mut().send_event(ServerUpdateEvent(sync_update_h1_completed));
    app.update();

    {
        let completed_scores = app.world().resource::<LeaderboardCompletedHolesScore>();
        assert_eq!(completed_scores.player_par_scores, vec![1, -1], "Completed Hole 1 should update par scores to +1 and -1");
        assert_eq!(completed_scores.last_completed_hole, 1);
    }

    // 3. Transition to Hole 2 AwaitingTurn with new running strokes: ticker should keep the completed Hole 1 scores.
    let sync_update_h2 = ServerUpdate::StateSync {
        sequence: 3,
        game_state: GameStateEnum::AwaitingTurn,
        active_player_id: 1234,
        current_hole: 2,
        player_positions: {
            let mut v = heapless::Vec::new();
            v.push(0).unwrap();
            v.push(0).unwrap();
            v
        },
        player_directions: {
            let mut v = heapless::Vec::new();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v
        },
        player_scores: {
            let mut v = heapless::Vec::new();
            v.push(Scorecard {
                running_strokes: 2, // 2 strokes on Hole 2 (should not affect completed score)
                total_strokes: 9,
                earned_cards: heapless::Vec::new(),
                cards_earned_this_hole: heapless::Vec::new(),
                strokes_per_hole: heapless::Vec::new(),
            }).unwrap();
            v.push(Scorecard {
                running_strokes: 1, // 1 stroke on Hole 2 (should not affect completed score)
                total_strokes: 6,
                earned_cards: heapless::Vec::new(),
                cards_earned_this_hole: heapless::Vec::new(),
                strokes_per_hole: heapless::Vec::new(),
            }).unwrap();
            v
        },
        placed_wagers: heapless::Vec::new(),
    };

    app.world_mut().send_event(ServerUpdateEvent(sync_update_h2));
    app.update();

    {
        let completed_scores = app.world().resource::<LeaderboardCompletedHolesScore>();
        assert_eq!(completed_scores.player_par_scores, vec![1, -1], "Hole 2 running strokes should not modify the leaderboard yet");
    }

    // Check that children were spawned inside the track node
    let track_entity = track_query.get_single(app.world()).unwrap();
    let children = app.world().get::<Children>(track_entity).expect("Track should have spawned children");
    assert_eq!(children.len(), 2, "Expected 2 player pill items in track");
}

#[test]
fn test_capsule_geometry_calculations() {
    use client::ui::systems::simulation::board::geometry::{calculate_capsule_layout, TrackGeometry};

    let viewport = Vec2::new(400.0, 300.0);
    let total_cells = 28;

    // Test TeeBox (first cell, idx 0.0)
    let layout_tee = calculate_capsule_layout(0.0, total_cells, viewport);
    // Tee should be on the left vertical segment, at the bottom
    assert!(layout_tee.position.x < 0.0, "Tee should be on the left segment");
    assert!(layout_tee.position.y < 0.0, "Tee should be at the bottom of the left segment");
    assert!((layout_tee.rotation_angle - std::f32::consts::PI).abs() < 1e-5, "Tee rotation angle should face perpendicular outwards (left)");

    // Test a cell on the top segment (e.g. index 10.0)
    let layout_top = calculate_capsule_layout(10.0, total_cells, viewport);
    // Top cell should be on the top segment (going left to right)
    assert!(layout_top.position.y > 0.0, "Top cell should be on top segment");
    assert!((layout_top.rotation_angle - std::f32::consts::FRAC_PI_2).abs() < 1e-5, "Top segment rotation should face perpendicular outwards (up)");

    // Test portrait viewport (e.g. 300x400)
    let viewport_portrait = Vec2::new(300.0, 400.0);
    let layout_tee_portrait = calculate_capsule_layout(0.0, total_cells, viewport_portrait);
    // Tee should still be on the left vertical segment, at the bottom
    assert!(layout_tee_portrait.position.x < 0.0, "Portrait Tee should be on the left segment");
    assert!(layout_tee_portrait.position.y < 0.0, "Portrait Tee should be at the bottom of the left segment");
    assert!((layout_tee_portrait.rotation_angle - std::f32::consts::PI).abs() < 1e-5, "Portrait Tee rotation should face perpendicular outwards (left)");

    // Test extreme viewports and aspect ratio compliance
    let viewports = vec![
        Vec2::new(10.0, 10.0),
        Vec2::new(100.0, 800.0),
        Vec2::new(1920.0, 400.0),
    ];
    for vp in viewports {
        let geom = TrackGeometry::calculate(vp);
        let ratio = geom.outer_width / geom.outer_height;
        assert!((ratio - 0.85).abs() < 1e-4, "Expected aspect ratio of 0.85 for viewport {:?}", vp);
    }
}

#[test]
fn test_banana_slide_click_interaction() {
    let mut app = setup_headless_ui_app();
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

    // Set ClientGameState state to BananaChoice
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<client::replication::ClientGameState>>();
        next_state.set(client::replication::ClientGameState::BananaChoice);
    }
    app.update();

    // Verify state transitioned
    {
        let state = app.world().resource::<State<client::replication::ClientGameState>>();
        assert_eq!(*state.get(), client::replication::ClientGameState::BananaChoice);
    }

    // Set ActivePlayerId and spawn a Ball entity
    let active_id = 9999;
    app.world_mut().insert_resource(client::replication::ActivePlayerId(active_id));
    app.world_mut().spawn((
        client::replication::Player { player_id: active_id },
        client::replication::Ball {
            cell_index: 11, // space 11
            direction: protocol::physics::MovementDirection::Forward,
            origin_cell: 11,
        },
        Transform::default(),
        GlobalTransform::default(),
    ));

    // Cell G0 (index 15) is reachable by moving 4 spaces
    // Find G0's position
    let cell_pos = {
        let mut cell_query = app.world_mut().query::<(&BoardCellNode, &Transform)>();
        let (_, cell_transform) = cell_query
            .iter(app.world())
            .find(|(node, _)| node.index == 15)
            .expect("Board cell with index 15 (G0) not found");
        cell_transform.translation.xy()
    };

    // Set cursor position override resource
    app.insert_resource(client::ui::components::CursorPositionOverride(Some(cell_pos)));

    // Simulate clicking the board cell
    {
        let mut mouse_input = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
        mouse_input.press(MouseButton::Left);
    }

    // Run the click handler system directly
    use bevy::ecs::system::RunSystemOnce;
    use client::ui::systems::simulation::board::interaction::handle_board_clicks_system;
    app.world_mut().run_system_once(handle_board_clicks_system);

    // Verify that ClientActionRequest event for ChooseBananaSlide was dispatched with step_count = 4
    let events = app.world().resource::<Events<ClientActionRequest>>();
    let mut reader = events.get_reader();
    let sent_events: Vec<&ClientActionRequest> = reader.read(events).collect();

    let choose_slide_event = sent_events.iter().find(|event| matches!(event.0, ClientAction::ChooseBananaSlide { .. }));
    assert!(choose_slide_event.is_some(), "Expected a ClientAction::ChooseBananaSlide event to be sent");
    if let Some(ClientActionRequest(ClientAction::ChooseBananaSlide { step_count })) = choose_slide_event {
        assert_eq!(*step_count, 4, "Expected step_count to be 4");
    } else {
        panic!("Sent event was not a ClientAction::ChooseBananaSlide variant");
    }
}

#[test]
fn test_match_completed_scorecard_screen() {
    use protocol::messages::{Scorecard, GameStateEnum};
    use client::network::ServerUpdateEvent;
    use client::replication::ClientGameState;

    let mut app = setup_headless_ui_app();
    app.update();

    let mut strokes_history = heapless::Vec::new();
    for _ in 0..18 {
        strokes_history.push(4u16).unwrap();
    }

    let scorecard = Scorecard {
        running_strokes: 0,
        total_strokes: 72,
        earned_cards: heapless::Vec::new(),
        cards_earned_this_hole: heapless::Vec::new(),
        strokes_per_hole: strokes_history,
    };

    let mut player_scores = heapless::Vec::new();
    player_scores.push(scorecard).unwrap();

    let sync_update = ServerUpdate::StateSync {
        sequence: 100,
        game_state: GameStateEnum::MatchCompleted,
        active_player_id: 1234,
        current_hole: 19,
        player_positions: {
            let mut v = heapless::Vec::new();
            v.push(0).unwrap();
            v
        },
        player_directions: {
            let mut v = heapless::Vec::new();
            v.push(protocol::physics::MovementDirection::Forward).unwrap();
            v
        },
        player_scores,
        placed_wagers: heapless::Vec::new(),
    };

    app.world_mut().send_event(ServerUpdateEvent(sync_update));
    app.update();
    app.update();
    app.update();

    let client_state = app.world().resource::<State<ClientGameState>>();
    assert_eq!(*client_state.get(), ClientGameState::MatchCompleted);

    let mut summary_style_query = app.world_mut().query_filtered::<&Style, With<client::ui::components::MatchCompletedScreenNode>>();
    let style = summary_style_query.get_single(app.world()).expect("MatchCompletedScreenNode missing");
    assert_eq!(style.display, Display::Flex);

    // Verify title text is MATCH COMPLETED
    {
        let mut title_query = app.world_mut().query_filtered::<&Text, With<client::ui::components::ScorecardTitleTextNode>>();
        let title_text = title_query.get_single(app.world()).unwrap();
        assert_eq!(title_text.sections[0].value, "MATCH COMPLETED");
    }

    let mut cell_query = app.world_mut().query::<(&Text, &client::ui::components::ScorecardCellTextNode)>();
    let cells: Vec<(&Text, &client::ui::components::ScorecardCellTextNode)> = cell_query.iter(app.world()).collect();
    assert!(!cells.is_empty(), "Expected scorecard cells to be populated");

    let total_cell = cells.iter().find(|(_, cell)| cell.hole_num == 22);
    assert!(total_cell.is_some(), "Total strokes cell (hole 22) not found");
    if let Some((text, _)) = total_cell {
        assert!(text.sections[0].value.contains("TOTAL STROKES: 72"), "Expected total strokes text to show 72, got: {}", text.sections[0].value);
    }
}

#[test]
fn test_in_progress_scorecard_toggle() {
    let mut app = setup_headless_ui_app();
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

    // Verify initial screen layout (HUD visible, MatchSummary hidden)
    {
        let summary_style = app.world_mut().query_filtered::<&Style, With<MatchCompletedScreenNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(summary_style.display, Display::None);

        let hud_style = app.world_mut().query_filtered::<&Style, With<TopHudNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(hud_style.display, Display::Flex);
    }

    // Simulate clicking the SCORECARD button
    {
        let scorecard_btn = app.world_mut().query_filtered::<Entity, With<ScorecardButtonNode>>()
            .get_single(app.world()).expect("Scorecard button missing");
        app.world_mut().entity_mut(scorecard_btn).insert(Interaction::Pressed);
    }
    app.update(); // trigger interactions
    app.update(); // trigger render and toggles

    // Verify ShowScorecard is true
    {
        let show_scorecard = app.world().resource::<ShowScorecard>();
        assert!(show_scorecard.0);
    }

    // Verify title matches active course name during play
    {
        let mut title_query = app.world_mut().query_filtered::<&Text, With<ScorecardTitleTextNode>>();
        let title_text = title_query.get_single(app.world()).unwrap();
        assert_eq!(title_text.sections[0].value, "GREEN COURSE");
    }

    // Verify screen layout toggled (HUD hidden, MatchSummary visible)
    {
        let summary_style = app.world_mut().query_filtered::<&Style, With<MatchCompletedScreenNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(summary_style.display, Display::Flex);

        let hud_style = app.world_mut().query_filtered::<&Style, With<TopHudNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(hud_style.display, Display::None);
    }

    // Verify button visibility context: BACK TO GAME is Flex, others are None
    {
        let close_btn_style = app.world_mut().query_filtered::<&Style, With<CloseScorecardButtonNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(close_btn_style.display, Display::Flex);

        let play_again_style = app.world_mut().query_filtered::<&Style, With<PlayAgainButtonNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(play_again_style.display, Display::None);

        let main_menu_style = app.world_mut().query_filtered::<&Style, With<MainMenuButtonNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(main_menu_style.display, Display::None);
    }

    // Simulate clicking the BACK TO GAME button
    {
        let close_btn = app.world_mut().query_filtered::<Entity, With<CloseScorecardButtonNode>>()
            .get_single(app.world()).expect("Close scorecard button missing");
        app.world_mut().entity_mut(close_btn).insert(Interaction::Pressed);
    }
    app.update(); // trigger interactions
    app.update(); // trigger render and toggles

    // Verify ShowScorecard is false and HUD is back to Flex
    {
        let show_scorecard = app.world().resource::<ShowScorecard>();
        assert!(!show_scorecard.0);

        let summary_style = app.world_mut().query_filtered::<&Style, With<MatchCompletedScreenNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(summary_style.display, Display::None);

        let hud_style = app.world_mut().query_filtered::<&Style, With<TopHudNode>>()
            .get_single(app.world()).unwrap();
        assert_eq!(hud_style.display, Display::Flex);
    }
}

#[test]
fn test_corner_cell_outer_boundary_click() {
    let mut app = setup_headless_ui_app();
    app.update();

    // Transition state to SoloSetup, then trigger PlayGameButton click to start game and spawn board
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<ClientScreenState>>();
        next_state.set(ClientScreenState::SoloSetup);
    }
    app.update();
    app.update();

    // Change current hole to Hole 5 (index 5)
    {
        let mut current_hole = app.world_mut().resource_mut::<client::ui::components::CurrentHole>();
        current_hole.0 = 5;
    }
    app.update();

    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<PlayGameButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Play Game button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }
    app.update();
    app.update();
    app.update();

    // Set ClientGameState to MarkerPlacement
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<client::replication::ClientGameState>>();
        next_state.set(client::replication::ClientGameState::MarkerPlacement);
    }
    app.update();

    // Select Banana wager card
    {
        let mut selected_card = app.world_mut().resource_mut::<SelectedWagerCard>();
        selected_card.0 = Some(CardType::Banana);
    }

    // Find cell 7 (which is curved corner tile 7 FW on Hole 5)
    let (cell_pos, cell_rot) = {
        let mut cell_query = app.world_mut().query::<(&BoardCellNode, &Transform)>();
        let (_, cell_transform) = cell_query
            .iter(app.world())
            .find(|(node, _)| node.index == 7)
            .expect("Board cell with index 7 (7 FW) not found");
        (cell_transform.translation.xy(), cell_transform.rotation)
    };

    // Calculate local X axis from rotation (perp direction)
    let local_x = (cell_rot * Vec3::X).xy().normalize();

    // Click at 75.0 units away from center (which exceeds the old 60.0 limit, but is within new 85.0 limit)
    let click_pos = cell_pos + local_x * 75.0;

    // Set cursor position override resource
    app.insert_resource(client::ui::components::CursorPositionOverride(Some(click_pos)));

    // Simulate clicking the board cell
    {
        let mut mouse_input = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
        mouse_input.press(MouseButton::Left);
    }

    // Run the clicks handler system directly
    use bevy::ecs::system::RunSystemOnce;
    use client::ui::systems::simulation::board::interaction::handle_board_clicks_system;
    app.world_mut().run_system_once(handle_board_clicks_system);

    // Verify that SelectedWagerCard resource was reset to None (meaning the click was accepted and drafted)
    {
        let selected_card = app.world().resource::<SelectedWagerCard>();
        assert_eq!(selected_card.0, None, "Expected SelectedWagerCard to be reset to None after successful click");
    }

    // Verify that ClientActionRequest event for DraftCard was dispatched with cell_index = 7
    let events = app.world().resource::<Events<ClientActionRequest>>();
    let mut reader = events.get_reader();
    let sent_events: Vec<&ClientActionRequest> = reader.read(events).collect();

    let draft_card_event = sent_events.iter().find(|event| matches!(event.0, ClientAction::DraftCard { .. }));
    assert!(draft_card_event.is_some(), "Expected a ClientAction::DraftCard event to be sent");
    if let Some(ClientActionRequest(ClientAction::DraftCard { card_type, cell_index })) = draft_card_event {
        assert_eq!(*card_type, CardType::Banana, "Expected card_type to be CardType::Banana");
        assert_eq!(*cell_index, 7, "Expected cell_index to be 7 (7 FW)");
    } else {
        panic!("Sent event was not a ClientAction::DraftCard variant");
    }
}

#[test]
fn test_touch_placement_interaction() {
    let mut app = setup_headless_ui_app();
    app.update();

    // Transition state to SoloSetup, then start game
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<ClientScreenState>>();
        next_state.set(ClientScreenState::SoloSetup);
    }
    app.update();
    app.update();

    // Set hole to 2
    {
        let mut current_hole = app.world_mut().resource_mut::<client::ui::components::CurrentHole>();
        current_hole.0 = 2;
    }
    app.update();

    {
        let mut button_query = app.world_mut().query_filtered::<Entity, With<PlayGameButtonNode>>();
        let button_entity = button_query.get_single(app.world()).expect("Play Game button missing");
        app.world_mut().entity_mut(button_entity).insert(Interaction::Pressed);
    }
    app.update();
    app.update();
    app.update();

    // Set ClientGameState to MarkerPlacement
    {
        let mut next_state = app.world_mut().resource_mut::<NextState<client::replication::ClientGameState>>();
        next_state.set(client::replication::ClientGameState::MarkerPlacement);
    }
    app.update();

    // Select Shield wager card
    {
        let mut selected_card = app.world_mut().resource_mut::<SelectedWagerCard>();
        selected_card.0 = Some(CardType::Shield);
    }

    // Find cell 1 (which is 1 RGH on Hole 2)
    let cell_pos = {
        let mut cell_query = app.world_mut().query::<(&BoardCellNode, &Transform)>();
        let (_, cell_transform) = cell_query
            .iter(app.world())
            .find(|(node, _)| node.index == 1)
            .expect("Board cell with index 1 (1 RGH) not found");
        cell_transform.translation.xy()
    };

    // Simulate touch event instead of mouse click
    {
        use bevy::input::touch::{TouchInput, TouchPhase};
        let window_entity = app.world_mut().query_filtered::<Entity, With<Window>>()
            .get_single(app.world()).expect("Window entity missing");

        let mut touch_events = app.world_mut().resource_mut::<Events<TouchInput>>();
        touch_events.send(TouchInput {
            phase: TouchPhase::Started,
            position: cell_pos,
            force: None,
            id: 0,
            window: window_entity,
        });
    }

    // Run update once to process events and update the Touches resource
    app.update();

    // Run the clicks handler system directly
    use bevy::ecs::system::RunSystemOnce;
    use client::ui::systems::simulation::board::interaction::handle_board_clicks_system;
    app.world_mut().run_system_once(handle_board_clicks_system);

    // Verify that SelectedWagerCard resource was reset to None (meaning the touch was accepted and drafted)
    {
        let selected_card = app.world().resource::<SelectedWagerCard>();
        assert_eq!(selected_card.0, None, "Expected SelectedWagerCard to be reset to None after successful touch");
    }

    // Verify that ClientActionRequest event for DraftCard was dispatched with cell_index = 1
    let events = app.world().resource::<Events<ClientActionRequest>>();
    let mut reader = events.get_reader();
    let sent_events: Vec<&ClientActionRequest> = reader.read(events).collect();

    let draft_card_event = sent_events.iter().find(|event| matches!(event.0, ClientAction::DraftCard { .. }));
    assert!(draft_card_event.is_some(), "Expected a ClientAction::DraftCard event to be sent via touch");
    if let Some(ClientActionRequest(ClientAction::DraftCard { card_type, cell_index })) = draft_card_event {
        assert_eq!(*card_type, CardType::Shield, "Expected card_type to be CardType::Shield");
        assert_eq!(*cell_index, 1, "Expected cell_index to be 1 (1 RGH)");
    } else {
        panic!("Sent event was not a ClientAction::DraftCard variant");
    }
}





