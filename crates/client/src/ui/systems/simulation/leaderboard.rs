use bevy::prelude::*;
use protocol::messages::{ServerUpdate, GameStateEnum};
use protocol::terrain::presets::get_course_preset;
use crate::network::ServerUpdateEvent;
use crate::ui::components::{
    GameSettings, LeaderboardTickerTrackNode, LeaderboardTickerContainerNode,
    LeaderboardCompletedHolesScore
};

pub fn update_leaderboard_ticker_system(
    mut commands: Commands,
    mut update_events: EventReader<ServerUpdateEvent>,
    settings: Res<GameSettings>,
    mut completed_scores: ResMut<LeaderboardCompletedHolesScore>,
    track_query: Query<Entity, With<LeaderboardTickerTrackNode>>,
) {
    for event in update_events.read() {
        if let ServerUpdate::StateSync {
            active_player_id,
            current_hole,
            player_positions,
            player_scores,
            game_state,
            ..
        } = &event.0
        {
            let num_players = player_positions.len();

            // 1. Reset completed scores at the start/lobby of the match
            if *current_hole <= 1 && *game_state != GameStateEnum::HoleCompleted {
                completed_scores.player_par_scores = vec![0; num_players];
                completed_scores.last_completed_hole = 0;
            }

            // 2. Accumulate scores only when a hole completion event happens
            if *game_state == GameStateEnum::HoleCompleted && *current_hole > completed_scores.last_completed_hole {
                if completed_scores.player_par_scores.len() < num_players {
                    completed_scores.player_par_scores.resize(num_players, 0);
                }

                let completed_hole_par = get_course_preset(&settings.course, *current_hole)
                    .map(|p| p.par)
                    .unwrap_or(4) as i32;

                for i in 0..num_players {
                    let strokes = player_scores.get(i).map(|s| s.running_strokes as i32).unwrap_or(0);
                    let relative_score = strokes - completed_hole_par;
                    completed_scores.player_par_scores[i] += relative_score;
                }
                completed_scores.last_completed_hole = *current_hole;
            }

            if let Ok(track_entity) = track_query.get_single() {
                // Clear old player entries
                commands.entity(track_entity).despawn_descendants();

                // Ensure par scores array size matches players count
                if completed_scores.player_par_scores.len() < num_players {
                    completed_scores.player_par_scores.resize(num_players, 0);
                }

                // Build players list
                let mut players = Vec::new();
                for i in 0..num_players {
                    let player_id = if i == 0 { *active_player_id } else { i as u64 };
                    let name = if i == 0 {
                        settings.nickname.clone()
                    } else {
                        format!("Bot {}", i)
                    };
                    let par_relative_score = completed_scores.player_par_scores[i];
                    players.push((player_id, name, par_relative_score));
                }

                // Sort by relative par score ascending
                players.sort_by_key(|p| p.2);

                // Spawn new player items inside the track
                commands.entity(track_entity).with_children(|track| {
                    for (rank, (player_id, name, par_relative_score)) in players.into_iter().enumerate() {
                        let rank_1based = (rank + 1) as u32;
                        let is_active = player_id == *active_player_id;

                        // Score relative to par representation
                        let score_str = if par_relative_score == 0 {
                            "E".to_string()
                        } else if par_relative_score > 0 {
                            format!("+{}", par_relative_score)
                        } else {
                            par_relative_score.to_string()
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
                track.scroll_offset += 30.0 * time.delta_seconds();
                if track.scroll_offset > track_width {
                    track.scroll_offset = -container_width;
                }
                style.left = Val::Px(-track.scroll_offset);
            } else {
                track.scroll_offset = 0.0;
                style.left = Val::Px(0.0);
            }
        }
    }
}
