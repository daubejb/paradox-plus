use bevy::prelude::*;
use protocol::messages::{ServerUpdate, GameStateEnum, CardType};
use protocol::terrain::presets::get_course_preset;
use protocol::terrain::TerrainType;
use crate::network::ServerUpdateEvent;
use crate::ui::components::{
    HoleTitleTextNode, HoleStatsTextNode, PlayerScoreTextNode, RollStatusTextNode,
    GameSettings, PlayerNameTextNode, RollOneButtonNode, RollTwoButtonNode,
    SkipPlacementButtonNode, WagerCardQtyTextNode
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
            ServerUpdate::StateSync { current_hole, player_scores, game_state, active_player_id, player_positions, placed_wagers, .. } => {
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
                        CardType::Shield => shield_count,
                        CardType::Banana => banana_count,
                        CardType::GoldenDie => die_count,
                    };
                    let label = match node.card_type {
                        CardType::Shield => "SHIELD",
                        CardType::Banana => "BANANA",
                        CardType::GoldenDie => "GOLDEN",
                    };
                    text.sections[0].value = format!("{} ({})", label, count);
                }

                let (roll_display, skip_display) = match game_state {
                    GameStateEnum::MarkerPlacement => (Display::None, Display::Flex),
                    GameStateEnum::AwaitingTurn => (Display::Flex, Display::None),
                    _ => (Display::None, Display::None),
                };

                let mut roll_two_display = roll_display;
                if roll_display == Display::Flex {
                    if let Some(preset) = get_course_preset(&settings.course, *current_hole) {
                        let active_pos = player_positions.first().copied().unwrap_or(0);
                        let terrain = preset.cells.get(active_pos as usize).copied().unwrap_or(TerrainType::Fairway);
                        let has_own_shield = placed_wagers.iter().any(|w| {
                            w.cell_index == active_pos
                                && w.card_type == CardType::Shield
                                && w.owner_id == *active_player_id
                        });
                        if terrain == TerrainType::Bunker || (terrain == TerrainType::Rough && !has_own_shield) {
                            roll_two_display = Display::None;
                        }
                    }
                }

                if let Ok(mut style) = roll_one_query.get_single_mut() {
                    style.display = roll_display;
                }
                if let Ok(mut style) = roll_two_query.get_single_mut() {
                    style.display = roll_two_display;
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
