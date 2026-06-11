use bevy::prelude::*;
use protocol::messages::{ServerUpdate, GameStateEnum};
use protocol::terrain::presets::get_course_preset;
use crate::network::ServerUpdateEvent;
use crate::ui::components::{HoleTitleTextNode, HoleStatsTextNode, PlayerScoreTextNode, RollStatusTextNode};

pub fn update_ui_elements_system(
    mut update_events: EventReader<ServerUpdateEvent>,
    mut title_query: Query<&mut Text, (With<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<PlayerScoreTextNode>, Without<RollStatusTextNode>)>,
    mut stats_query: Query<&mut Text, (With<HoleStatsTextNode>, Without<HoleTitleTextNode>, Without<PlayerScoreTextNode>, Without<RollStatusTextNode>)>,
    mut score_query: Query<&mut Text, (With<PlayerScoreTextNode>, Without<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<RollStatusTextNode>)>,
    mut status_query: Query<&mut Text, (With<RollStatusTextNode>, Without<HoleTitleTextNode>, Without<HoleStatsTextNode>, Without<PlayerScoreTextNode>)>,
) {
    for event in update_events.read() {
        match &event.0 {
            ServerUpdate::StateSync { current_hole, player_scores, game_state, .. } => {
                if let Some(preset) = get_course_preset("green", *current_hole) {
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

                if *game_state == GameStateEnum::HoleCompleted {
                    if let Ok(mut text) = status_query.get_single_mut() {
                        text.sections[0].value = "Completed Hole 1! Well played!".to_string();
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
