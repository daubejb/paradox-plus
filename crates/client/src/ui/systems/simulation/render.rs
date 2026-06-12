use bevy::prelude::*;
use protocol::messages::{ServerUpdate, GameStateEnum};
use protocol::terrain::presets::get_course_preset;
use crate::network::ServerUpdateEvent;
use crate::ui::components::{
    HoleTitleTextNode, HoleStatsTextNode, PlayerScoreTextNode, RollStatusTextNode,
    GameSettings, PlayerNameTextNode, RollOneButtonNode, RollTwoButtonNode,
    SkipPlacementButtonNode, WagerCardQtyTextNode, LeaderboardTickerTrackNode,
    LeaderboardTickerContainerNode
};

pub fn update_ui_elements_system(
    mut update_events: EventReader<ServerUpdateEvent>,
    settings: Res<GameSettings>,
    mut title_query: Query<&mut Text, (With<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<PlayerScoreTextNode>, Without<RollStatusTextNode>, Without<PlayerNameTextNode>, Without<WagerCardQtyTextNode>)>,
    mut stats_query: Query<&mut Text, (With<HoleStatsTextNode>, Without<HoleTitleTextNode>, Without<PlayerScoreTextNode>, Without<RollStatusTextNode>, Without<PlayerNameTextNode>, Without<WagerCardQtyTextNode>)>,
    mut score_query: Query<&mut Text, (With<PlayerScoreTextNode>, Without<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<RollStatusTextNode>, Without<PlayerNameTextNode>, Without<WagerCardQtyTextNode>)>,
    mut status_query: Query<&mut Text, (With<RollStatusTextNode>, Without<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<PlayerScoreTextNode>, Without<PlayerNameTextNode>, Without<WagerCardQtyTextNode>)>,
    mut name_query: Query<&mut Text, (With<PlayerNameTextNode>, Without<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<PlayerScoreTextNode>, Without<RollStatusTextNode>, Without<WagerCardQtyTextNode>)>,
    mut qty_query: Query<(&mut Text, &WagerCardQtyTextNode), (Without<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<PlayerScoreTextNode>, Without<RollStatusTextNode>, Without<PlayerNameTextNode>)>,
    mut roll_one_query: Query<&mut Style, (With<RollOneButtonNode>, Without<RollTwoButtonNode>, Without<SkipPlacementButtonNode>)>,
    mut roll_two_query: Query<&mut Style, (With<RollTwoButtonNode>, Without<RollOneButtonNode>, Without<SkipPlacementButtonNode>)>,
    mut skip_query: Query<&mut Style, (With<SkipPlacementButtonNode>, Without<RollOneButtonNode>, Without<RollTwoButtonNode>)>,
) {
    for event in update_events.read() {
        match &event.0 {
            ServerUpdate::StateSync { current_hole, player_scores, game_state, .. } => {
                if let Some(preset) = get_course_preset(&settings.course, *current_hole) {
                    if let Ok(mut text) = title_query.get_single_mut() {
                        text.sections[0].value = format!("HOLE {}", current_hole);
                    }
                    if let Ok(mut text) = stats_query.get_single_mut() {
                        text.sections[0].value = format!("PAR {} • {} SPACES", preset.par, preset.total_cells);
                    }
                }

                if let Ok(mut text) = score_query.get_single_mut() {
                    let running_strokes = player_scores.first().map(|s| s.running_strokes).unwrap_or(0);
                    text.sections[0].value = format!("🏆 {} strokes", running_strokes);
                }

                if let Ok(mut text) = name_query.get_single_mut() {
                    text.sections[0].value = settings.nickname.to_uppercase();
                }

                let score_val = player_scores.first();
                let shield_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 0).count()).unwrap_or(0);
                let banana_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 1).count()).unwrap_or(0);
                let die_count = score_val.map(|s| s.earned_cards.iter().filter(|&&c| c == 2).count()).unwrap_or(0);

                for (mut text, node) in qty_query.iter_mut() {
                    let count = match node.card_type {
                        0 => shield_count,
                        1 => banana_count,
                        _ => die_count,
                    };
                    let label = match node.card_type {
                        0 => "SHIELD",
                        1 => "BANANA",
                        _ => "GOLDEN",
                    };
                    text.sections[0].value = format!("{} ({})", label, count);
                }

                let (roll_display, skip_display) = match game_state {
                    GameStateEnum::MarkerPlacement => (Display::None, Display::Flex),
                    GameStateEnum::AwaitingTurn => (Display::Flex, Display::None),
                    _ => (Display::None, Display::None),
                };

                if let Ok(mut style) = roll_one_query.get_single_mut() {
                    style.display = roll_display;
                }
                if let Ok(mut style) = roll_two_query.get_single_mut() {
                    style.display = roll_display;
                }
                if let Ok(mut style) = skip_query.get_single_mut() {
                    style.display = skip_display;
                }

                if *game_state == GameStateEnum::HoleCompleted {
                    if let Ok(mut text) = status_query.get_single_mut() {
                        let score_val = player_scores.first();
                        let cards_str = if let Some(score) = score_val {
                            if score.earned_cards.is_empty() {
                                " No cards earned.".to_string()
                            } else {
                                let mut names = Vec::new();
                                for card in score.earned_cards.iter() {
                                    match card {
                                        0 => names.push("Guardian Shield"),
                                        1 => names.push("Trickster Banana"),
                                        _ => names.push("Golden Die"),
                                    }
                                }
                                format!(" Earned: {}", names.join(", "))
                            }
                        } else {
                            "".to_string()
                        };
                        text.sections[0].value = format!("Completed Hole {}! Well played!{}", current_hole, cards_str);
                    }
                }
            }
            ServerUpdate::DiceRollOutcome { roll_values } => {
                if let Ok(mut text) = status_query.get_single_mut() {
                    let sum: u8 = roll_values.iter().sum();
                    text.sections[0].value = format!("Rolled: {} {:?}", sum, roll_values);
                }
            }
            ServerUpdate::AlertTriggered { alert_message } => {
                if let Ok(mut text) = status_query.get_single_mut() {
                    text.sections[0].value = alert_message.to_string();
                }
            }
            _ => {}
        }
    }
}

pub fn update_leaderboard_ticker_system(
    mut commands: Commands,
    mut update_events: EventReader<ServerUpdateEvent>,
    settings: Res<GameSettings>,
    track_query: Query<Entity, With<LeaderboardTickerTrackNode>>,
) {
    for event in update_events.read() {
        if let ServerUpdate::StateSync {
            active_player_id,
            current_hole,
            player_positions,
            player_scores,
            ..
        } = &event.0
        {
            let current_hole_par = get_course_preset(&settings.course, *current_hole)
                .map(|p| p.par)
                .unwrap_or(4) as i32;

            if let Ok(track_entity) = track_query.get_single() {
                // Clear old player entries
                commands.entity(track_entity).despawn_descendants();

                // Build players list
                let mut players = Vec::new();
                for i in 0..player_positions.len() {
                    let player_id = if i == 0 { *active_player_id } else { i as u64 };
                    let name = if i == 0 {
                        settings.nickname.clone()
                    } else {
                        format!("Bot {}", i)
                    };
                    let strokes = player_scores.get(i).map(|s| s.running_strokes as i32).unwrap_or(0);
                    players.push((player_id, name, strokes));
                }

                // Sort by strokes ascending (lowest total strokes is first/leader)
                players.sort_by_key(|p| p.2);

                // Spawn new player items inside the track
                commands.entity(track_entity).with_children(|track| {
                    for (rank, (player_id, name, strokes)) in players.into_iter().enumerate() {
                        let rank_1based = (rank + 1) as u32;
                        let is_active = player_id == *active_player_id;

                        // Score relative to par
                        let score_str = if strokes == 0 {
                            "E".to_string()
                        } else {
                            let diff = strokes - current_hole_par;
                            if diff == 0 {
                                "E".to_string()
                            } else if diff > 0 {
                                format!("+{}", diff)
                            } else {
                                diff.to_string()
                            }
                        };

                        // 1st gets gold, 2nd silver, others dark badge
                        let rank_bg = match rank_1based {
                            1 => Color::srgb(0.85, 0.65, 0.15), // Gold
                            2 => Color::srgb(0.6, 0.65, 0.62),  // Silver
                            _ => Color::srgb(0.15, 0.22, 0.18), // Dark
                        };

                        // Pill background and outline
                        let (pill_bg, border_color) = if is_active {
                            (Color::srgb(0.08, 0.28, 0.14), Color::srgb(0.2, 0.65, 0.35)) // Highlight active player green
                        } else {
                            (Color::srgb(0.04, 0.08, 0.06), Color::srgb(0.1, 0.18, 0.14)) // Dark neutral
                        };

                        track.spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.0),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            background_color: pill_bg.into(),
                            border_color: border_color.into(),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        }).with_children(|pill| {
                            // Rank Circle Badge
                            pill.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(18.0),
                                    height: Val::Px(18.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: rank_bg.into(),
                                border_radius: BorderRadius::all(Val::Px(9.0)),
                                ..default()
                            }).with_children(|circle| {
                                circle.spawn(TextBundle::from_section(
                                    rank_1based.to_string(),
                                    TextStyle {
                                        font_size: 9.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                            });

                            // Player Name Text
                            pill.spawn(TextBundle::from_section(
                                name.clone(),
                                TextStyle {
                                    font_size: 11.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));

                            // Score Text
                            pill.spawn(TextBundle::from_section(
                                score_str,
                                TextStyle {
                                    font_size: 11.0,
                                    color: Color::srgb(0.9, 0.8, 0.2), // Gold score
                                    ..default()
                                },
                            ));
                        });
                    }
                });
            }
        }
    }
}

pub fn scroll_leaderboard_ticker_system(
    time: Res<Time>,
    mut track_query: Query<(&mut Style, &mut LeaderboardTickerTrackNode, &Node, &Parent)>,
    container_query: Query<&Node, With<LeaderboardTickerContainerNode>>,
) {
    if let Ok((mut style, mut track, track_node, parent)) = track_query.get_single_mut() {
        if let Ok(container_node) = container_query.get(parent.get()) {
            let track_width = track_node.size().x;
            let container_width = container_node.size().x;

            if track_width > container_width && container_width > 0.0 {
                // Content overflows container, let's scroll!
                // Speed: 30 pixels per second
                track.scroll_offset += 30.0 * time.delta_seconds();

                // When track scrolls fully out of view, reset it to the right edge
                // so it loops seamlessly.
                if track.scroll_offset > track_width {
                    track.scroll_offset = -container_width;
                }

                style.left = Val::Px(-track.scroll_offset);
            } else {
                // Fits horizontally, no scrolling needed
                track.scroll_offset = 0.0;
                style.left = Val::Px(0.0);
            }
        }
    }
}
