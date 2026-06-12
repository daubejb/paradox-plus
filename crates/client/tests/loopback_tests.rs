use bevy::prelude::*;
use client::ui::systems::simulation::loopback::{
    local_offline_server_system,
    LocalServerChannels,
    state::OfflineServerState,
};
use protocol::messages::{ClientAction, GameStateEnum, WagerToken};

#[test]
fn test_loopback_hole_transition_marker_placement() {
    let mut app = App::new();

    // 1. Setup mock channels
    let (_action_tx, action_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (update_tx, _update_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    app.insert_resource(LocalServerChannels {
        action_rx: std::sync::Mutex::new(action_rx),
        update_tx,
        send_buf: std::sync::Mutex::new(Vec::with_capacity(65536)),
    });

    // 2. Setup OfflineServerState with wager mode enabled and cards in inventory
    let mut state = OfflineServerState::default();
    state.is_wager_mode = true;
    state.inventory = vec![1]; // Player has a Banana card
    state.game_state = GameStateEnum::HoleCompleted;
    state.hole_completed_timer_ms = Some(2900); // 100ms away from triggering transition
    state.placed_wagers = vec![WagerToken {
        card_type: 1,
        owner_id: 1234,
        cell_index: 5,
    }];
    app.insert_resource(state);

    // 3. Manually insert Time resource with explicit Type
    let mut time = Time::<()>::default();
    time.advance_by(std::time::Duration::from_millis(0));
    app.insert_resource(time);

    // Add system to Update schedule
    app.add_systems(Update, local_offline_server_system);

    // Update time by 50ms (timer goes to 2950, shouldn't trigger transition yet)
    {
        let mut time = app.world_mut().resource_mut::<Time<()>>();
        time.advance_by(std::time::Duration::from_millis(50));
    }
    app.update();

    {
        let state = app.world().resource::<OfflineServerState>();
        assert_eq!(state.game_state, GameStateEnum::HoleCompleted);
        assert_eq!(state.placed_wagers.len(), 1, "Wagers should not be cleared yet");
    }

    // Update time by another 100ms (timer goes to 3050, triggering hole transition)
    {
        let mut time = app.world_mut().resource_mut::<Time<()>>();
        time.advance_by(std::time::Duration::from_millis(100));
    }
    app.update();

    {
        let state = app.world().resource::<OfflineServerState>();
        // Advanced from Hole 1 to Hole 2
        assert_eq!(state.current_hole, 2);
        // Position and strokes reset
        assert_eq!(state.player_position, 0);
        assert_eq!(state.strokes, 0);
        // Wagers cleared
        assert!(state.placed_wagers.is_empty(), "Wagers must be cleared on hole transition");
        // Transitions to MarkerPlacement since is_wager_mode is true and inventory has cards
        assert_eq!(state.game_state, GameStateEnum::MarkerPlacement);
    }
}

#[test]
fn test_loopback_hole_transition_awaiting_turn_if_no_cards() {
    let mut app = App::new();

    let (_action_tx, action_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (update_tx, _update_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    app.insert_resource(LocalServerChannels {
        action_rx: std::sync::Mutex::new(action_rx),
        update_tx,
        send_buf: std::sync::Mutex::new(Vec::with_capacity(65536)),
    });

    // Wager mode is enabled but player has no cards in inventory
    let mut state = OfflineServerState::default();
    state.is_wager_mode = true;
    state.inventory = vec![];
    state.game_state = GameStateEnum::HoleCompleted;
    state.hole_completed_timer_ms = Some(3000); // Trigger immediately
    app.insert_resource(state);

    app.insert_resource(Time::<()>::default());
    app.add_systems(Update, local_offline_server_system);

    app.update();

    {
        let state = app.world().resource::<OfflineServerState>();
        assert_eq!(state.current_hole, 2);
        // Transitions to AwaitingTurn since inventory is empty
        assert_eq!(state.game_state, GameStateEnum::AwaitingTurn);
    }
}

#[test]
fn test_loopback_ignores_roll_during_hole_completed() {
    let mut app = App::new();

    let (action_tx, action_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (update_tx, _update_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    app.insert_resource(LocalServerChannels {
        action_rx: std::sync::Mutex::new(action_rx),
        update_tx,
        send_buf: std::sync::Mutex::new(Vec::with_capacity(65536)),
    });

    let mut state = OfflineServerState::default();
    state.game_state = GameStateEnum::HoleCompleted;
    state.player_position = 23;
    state.is_initialized = true;
    app.insert_resource(state);

    app.insert_resource(Time::<()>::default());
    app.add_systems(Update, local_offline_server_system);

    // Send RollDice action which should be invalid in HoleCompleted state
    let action = protocol::messages::ClientAction::RollDice { dice_count: 1 };
    let mut buf = [0u8; 1024];
    let serialized = postcard::to_slice(&action, &mut buf).unwrap();
    action_tx.send(serialized.to_vec()).unwrap();

    app.update();

    {
        let state = app.world().resource::<OfflineServerState>();
        // Verify state did not change and roll was ignored
        assert_eq!(state.game_state, GameStateEnum::HoleCompleted);
        assert_eq!(state.player_position, 23);
    }
}

#[test]
fn test_loopback_wager_card_earning_logic() {
    use protocol::terrain::presets::get_course_preset;
    use client::ui::systems::simulation::loopback::handlers::handle_action;

    let course = get_course_preset("green", 2).unwrap(); // Hole 2 is Par 3

    // Scenario A: Birdie (Total 2 strokes on Par 3)
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 2;
        state.player_position = 4; // close to green
        state.strokes = 0; // Will complete in 2 strokes if we roll a 3 (lands on Green 1, which has 1 putt)
        state.is_initialized = true;

        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 1000 {
                panic!("Failed to roll Birdie in 1000 attempts");
            }
            let mut test_state = state.clone();
            let _updates = handle_action(&mut test_state, &ClientAction::RollDice { dice_count: 1 }, &course);
            if test_state.game_state == GameStateEnum::HoleCompleted {
                if test_state.strokes == 2 {
                    assert_eq!(test_state.inventory.len(), 1, "Birdie (2 strokes on Par 3) must earn exactly 1 card");
                    break;
                }
            }
        }
    }

    // Scenario B: Eagle (Total 1 stroke on Par 3)
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 2;
        state.player_position = 4;
        state.strokes = 0; // Will complete in 1 stroke if we roll a 4 (lands on Green 0, which has 0 putts)
        state.is_initialized = true;

        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 1000 {
                panic!("Failed to roll Eagle in 1000 attempts");
            }
            let mut test_state = state.clone();
            let _updates = handle_action(&mut test_state, &ClientAction::RollDice { dice_count: 1 }, &course);
            if test_state.game_state == GameStateEnum::HoleCompleted {
                if test_state.strokes == 1 {
                    assert_eq!(test_state.inventory.len(), 1, "Eagle (1 stroke on Par 3) must earn exactly 1 card");
                    break;
                }
            }
        }
    }

    // Scenario C: Par (Total 3 strokes on Par 3)
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 2;
        state.player_position = 4;
        state.strokes = 1; // Will complete in 3 strokes if we roll a 3 (1 initial + 1 approach + 1 putt)
        state.is_initialized = true;

        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 1000 {
                panic!("Failed to roll Par in 1000 attempts");
            }
            let mut test_state = state.clone();
            let _updates = handle_action(&mut test_state, &ClientAction::RollDice { dice_count: 1 }, &course);
            if test_state.game_state == GameStateEnum::HoleCompleted {
                if test_state.strokes == 3 {
                    assert_eq!(test_state.inventory.len(), 0, "Par (3 strokes on Par 3) must earn 0 cards");
                    break;
                }
            }
        }
    }

    // Scenario D: Bogey (Total 4 strokes on Par 3)
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 2;
        state.player_position = 4;
        state.strokes = 2; // Will complete in 4 strokes if we roll a 3 (2 initial + 1 approach + 1 putt)
        state.is_initialized = true;

        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 1000 {
                panic!("Failed to roll Bogey in 1000 attempts");
            }
            let mut test_state = state.clone();
            let _updates = handle_action(&mut test_state, &ClientAction::RollDice { dice_count: 1 }, &course);
            if test_state.game_state == GameStateEnum::HoleCompleted {
                if test_state.strokes == 4 {
                    assert_eq!(test_state.inventory.len(), 0, "Bogey (4 strokes on Par 3) must earn 0 cards");
                    break;
                }
            }
        }
    }
}

#[test]
fn test_loopback_landing_on_golden_die() {
    use protocol::terrain::presets::get_course_preset;
    use client::ui::systems::simulation::loopback::handlers::handle_action;

    let course = get_course_preset("green", 1).unwrap(); // Hole 1

    let mut state = OfflineServerState::default();
    state.is_wager_mode = true;
    state.current_hole = 1;
    state.player_position = 0; // Tee
    state.strokes = 3;
    state.inventory = vec![2]; // Golden Die card (card_type = 2)
    state.is_initialized = true;

    // 1. Place the Golden Die wager on cell index 6 (Fairway)
    let _updates = handle_action(
        &mut state,
        &ClientAction::DraftCard {
            card_type: 2,
            cell_index: 6,
        },
        &course,
    );

    // Verify it was placed and removed from inventory
    assert!(state.inventory.is_empty(), "Inventory should be empty after drafting the card");
    assert_eq!(state.placed_wagers.len(), 1, "There should be 1 placed wager");
    assert_eq!(state.placed_wagers[0].cell_index, 6);

    // 2. Roll dice and filter until we get a roll sum of exactly 6 (landing on index 6)
    let mut attempts = 0;
    loop {
        attempts += 1;
        if attempts > 1000 {
            panic!("Failed to roll a 6 in 1000 attempts");
        }
        let mut test_state = state.clone();
        let roll_updates = handle_action(
            &mut test_state,
            &ClientAction::RollDice { dice_count: 1 },
            &course,
        );

        // Find the DiceRollOutcome sum
        let mut rolled_6 = false;
        for update in &roll_updates {
            if let protocol::messages::ServerUpdate::DiceRollOutcome { roll_values } = update {
                let sum: u8 = roll_values.iter().sum();
                if sum == 6 {
                    rolled_6 = true;
                }
            }
        }

        if rolled_6 {
            // Verify that the player landed on cell index 6
            assert_eq!(test_state.player_position, 6);
            // Verify strokes: we had 3, rolled (adds 1 stroke for the shot), then Golden Die subtracts 2.
            // Expected strokes: 3 + 1 - 2 = 2 strokes.
            assert_eq!(test_state.strokes, 2, "Strokes should be 2 after -2 reduction");
            // Verify inventory: player should have earned a new Golden Die card (2) back
            assert_eq!(test_state.inventory.len(), 1, "Should have earned a card");
            assert_eq!(test_state.inventory[0], 2, "Earned card should be Golden Die");

            // Verify that an alert update was generated
            let has_alert = roll_updates.iter().any(|update| {
                if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = update {
                    alert_message.contains("Triggered Golden Die")
                } else {
                    false
                }
            });
            assert!(has_alert, "Should trigger Golden Die alert");
            break;
        }
    }
}

#[test]
fn test_loopback_wager_placement_validations() {
    use protocol::terrain::presets::get_course_preset;
    use client::ui::systems::simulation::loopback::handlers::handle_action;

    // Green Course Hole 1:
    // (6, &["r", "r", "r", "r", "r", "f", "f", "f", "f", "f", "f", "f", "f", "s", "s", "f", "f", "f", "f", "g3", "g2", "g1", "g0", "g1", "g2", "f"])
    let green_course = get_course_preset("green", 1).unwrap();

    // 1. Placement on Green should fail
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.inventory = vec![2]; // Golden Die
        state.is_initialized = true;

        let updates = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 2,
                cell_index: 22, // Green cup
            },
            &green_course,
        );

        // Verify rejection
        assert_eq!(state.inventory.len(), 1, "Card should remain in inventory");
        assert!(state.placed_wagers.is_empty(), "Wager should not be placed");
        
        let has_err = updates.iter().any(|u| {
            if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                alert_message.contains("Cannot place wager on Green")
            } else {
                false
            }
        });
        assert!(has_err, "Should trigger Green placement error alert");
    }

    // 2. Placement on Tee Box should fail
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.inventory = vec![2]; // Golden Die
        state.is_initialized = true;

        let updates = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 2,
                cell_index: 0, // Tee Box
            },
            &green_course,
        );

        // Verify rejection
        assert_eq!(state.inventory.len(), 1);
        assert!(state.placed_wagers.is_empty());
        
        let has_err = updates.iter().any(|u| {
            if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                alert_message.contains("Cannot place wager on Tee Box")
            } else {
                false
            }
        });
        assert!(has_err, "Should trigger Tee Box error alert");
    }

    // 3. Shield on non-hazard (Fairway) should fail
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.inventory = vec![0]; // Guardian Shield
        state.is_initialized = true;

        let updates = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 0,
                cell_index: 6, // Fairway (non-hazard)
            },
            &green_course,
        );

        assert_eq!(state.inventory.len(), 1);
        assert!(state.placed_wagers.is_empty());
        
        let has_err = updates.iter().any(|u| {
            if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                alert_message.contains("Guardian Shield must be placed on a Hazard")
            } else {
                false
            }
        });
        assert!(has_err, "Should trigger Shield hazard error alert");
    }

    // 4. Golden Die on Hazard should fail
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.inventory = vec![2]; // Golden Die
        state.is_initialized = true;

        let updates = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 2,
                cell_index: 14, // Sand Bunker
            },
            &green_course,
        );

        assert_eq!(state.inventory.len(), 1);
        assert!(state.placed_wagers.is_empty());
        
        let has_err = updates.iter().any(|u| {
            if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                alert_message.contains("Golden Die must be placed on Fairway")
            } else {
                false
            }
        });
        assert!(has_err, "Should trigger Golden Die fairway error alert");
    }

    // 5. Banana 4 spaces before OB should fail
    // Blue Course Hole 1:
    // (5, &["r", "f", "f", "f", "f", "f", "lb", ...])
    // Cell 3 is fairway, Cell 7 is OB ("lb")
    {
        let blue_course = get_course_preset("blue", 1).unwrap();
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.inventory = vec![1]; // Trickster Banana
        state.is_initialized = true;

        let updates = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 1,
                cell_index: 3, // Fairway, 4 spaces before OB at cell 7 (3 + 4 = 7)
            },
            &blue_course,
        );

        assert_eq!(state.inventory.len(), 1);
        assert!(state.placed_wagers.is_empty());
        
        let has_err = updates.iter().any(|u| {
            if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                alert_message.contains("Trickster Banana cannot be placed 4 spaces before OB")
            } else {
                false
            }
        });
        assert!(has_err, "Should trigger Banana OB error alert");
    }

    // 6. Overlapping wagers on the same tile should fail
    {
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.inventory = vec![2, 2]; // Two Golden Die cards
        state.is_initialized = true;

        // Place first card
        let _ = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 2,
                cell_index: 6, // Fairway
            },
            &green_course,
        );
        assert_eq!(state.placed_wagers.len(), 1);

        // Place second card on same cell
        let updates = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 2,
                cell_index: 6,
            },
            &green_course,
        );

        assert_eq!(state.inventory.len(), 1, "Second card should remain in inventory");
        assert_eq!(state.placed_wagers.len(), 1, "Only 1 wager should remain placed");
        
        let has_err = updates.iter().any(|u| {
            if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                alert_message.contains("Tile already has a wager token placed")
            } else {
                false
            }
        });
        assert!(has_err, "Should trigger overlap error alert");
    }
}

#[test]
fn test_loopback_landing_on_shield_prophecy() {
    use protocol::terrain::presets::get_course_preset;
    use client::ui::systems::simulation::loopback::handlers::handle_action;

    // Test A: Shield on Rough
    {
        let course = get_course_preset("green", 1).unwrap();
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.player_position = 0;
        state.strokes = 0;
        state.inventory = vec![0]; // Guardian Shield
        state.is_initialized = true;

        // Draft Shield on cell index 5 (Rough)
        let _ = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 0,
                cell_index: 5,
            },
            &course,
        );
        assert_eq!(state.placed_wagers.len(), 1);

        // Roll 5 to land on index 5
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 1000 {
                panic!("Failed to roll a 5 in 1000 attempts");
            }
            let mut test_state = state.clone();
            let updates = handle_action(
                &mut test_state,
                &ClientAction::RollDice { dice_count: 1 },
                &course,
            );

            let mut rolled_5 = false;
            for update in &updates {
                if let protocol::messages::ServerUpdate::DiceRollOutcome { roll_values } = update {
                    if roll_values.iter().sum::<u8>() == 5 {
                        rolled_5 = true;
                    }
                }
            }

            if rolled_5 {
                assert_eq!(test_state.player_position, 5);
                assert_eq!(test_state.strokes, 1, "Strokes should be exactly 1 (0 penalty)");
                assert_eq!(test_state.inventory.len(), 1, "Should have drawn a card");
                assert!(updates.iter().any(|u| {
                    if let protocol::messages::ServerUpdate::AlertTriggered { alert_message } = u {
                        alert_message.contains("Shield triggered")
                    } else {
                        false
                    }
                }));
                break;
            }
        }
    }

    // Test B: Shield on Out-of-Bounds (Lost Ball)
    {
        let blue_course = get_course_preset("blue", 1).unwrap();
        let mut state = OfflineServerState::default();
        state.is_wager_mode = true;
        state.current_hole = 1;
        state.player_position = 1; // Start on cell 1 (Rough)
        state.strokes = 0;
        state.inventory = vec![0]; // Guardian Shield
        state.is_initialized = true;

        // Draft Shield on cell index 7 (OB / Lost Ball)
        let _ = handle_action(
            &mut state,
            &ClientAction::DraftCard {
                card_type: 0,
                cell_index: 7,
            },
            &blue_course,
        );
        assert_eq!(state.placed_wagers.len(), 1);

        // Roll 6 to land on index 7 (from index 1, 1 + 6 = 7)
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 1000 {
                panic!("Failed to roll a 6 in 1000 attempts");
            }
            let mut test_state = state.clone();
            let updates = handle_action(
                &mut test_state,
                &ClientAction::RollDice { dice_count: 1 },
                &blue_course,
            );

            let mut rolled_6 = false;
            for update in &updates {
                if let protocol::messages::ServerUpdate::DiceRollOutcome { roll_values } = update {
                    if roll_values.iter().sum::<u8>() == 6 {
                        rolled_6 = true;
                    }
                }
            }

            if rolled_6 {
                // Should stay on cell 7 (neutralized hazard) and NOT reset to origin cell 1
                assert_eq!(test_state.player_position, 7, "Player should stay on cell 7");
                assert_eq!(test_state.strokes, 1, "Strokes should be exactly 1 (0 penalty, no OB reset)");
                assert_eq!(test_state.inventory.len(), 1, "Should have drawn a card");
                break;
            }
        }
    }
}

#[test]
fn test_loopback_landing_on_banana_prophecy() {
    use protocol::terrain::presets::get_course_preset;
    use client::ui::systems::simulation::loopback::handlers::handle_action;

    let course = get_course_preset("green", 1).unwrap();
    let mut state = OfflineServerState::default();
    state.is_wager_mode = true;
    state.current_hole = 1;
    state.player_position = 0;
    state.strokes = 0;
    state.inventory = vec![1]; // Trickster Banana
    state.is_initialized = true;

    // 1. Draft Banana on cell index 6 (Fairway)
    let _ = handle_action(
        &mut state,
        &ClientAction::DraftCard {
            card_type: 1,
            cell_index: 6,
        },
        &course,
    );
    assert_eq!(state.placed_wagers.len(), 1);

    // 2. Roll 6 to land on index 6
    let mut attempts = 0;
    loop {
        attempts += 1;
        if attempts > 1000 {
            panic!("Failed to roll a 6 in 1000 attempts");
        }
        let mut test_state = state.clone();
        let updates = handle_action(
            &mut test_state,
            &ClientAction::RollDice { dice_count: 1 },
            &course,
        );

        let mut rolled_6 = false;
        for update in &updates {
            if let protocol::messages::ServerUpdate::DiceRollOutcome { roll_values } = update {
                if roll_values.iter().sum::<u8>() == 6 {
                    rolled_6 = true;
                }
            }
        }

        if rolled_6 {
            assert_eq!(test_state.player_position, 6);
            assert_eq!(test_state.game_state, GameStateEnum::BananaChoice, "Game state must transition to BananaChoice");
            assert_eq!(test_state.inventory.len(), 1, "Should have drawn a card on landing");
            assert_eq!(test_state.strokes, 1, "Strokes should be 1 for the initial RollDice");

            // 3. Send ChooseBananaSlide to slide 3 spaces forward
            let slide_updates = handle_action(
                &mut test_state,
                &ClientAction::ChooseBananaSlide { step_count: 3 },
                &course,
            );

            // Verify movement to index 9 (6 + 3 = 9)
            assert_eq!(test_state.player_position, 9);
            // Verify game state transitions back to AwaitingTurn
            assert_eq!(test_state.game_state, GameStateEnum::AwaitingTurn);
            // Verify stroke count did not increase (stays at 1 stroke)
            assert_eq!(test_state.strokes, 1, "Slide movement should cost 0 strokes");
            assert!(slide_updates.iter().any(|u| {
                if let protocol::messages::ServerUpdate::StateSync { game_state, .. } = u {
                    *game_state == GameStateEnum::AwaitingTurn
                } else {
                    false
                }
            }));
            break;
        }
    }
}


