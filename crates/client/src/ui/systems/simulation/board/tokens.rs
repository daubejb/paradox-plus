use bevy::prelude::*;
use protocol::messages::CardType;
use crate::replication::Ball;
use crate::ui::components::{BoardCellNode, PlayerTokenMarker, WagerTokenMarker, ClientWagers};

/// Updates player ball location indicator on the 2D board.
pub fn update_board_cell_visuals(
    ball_query: Query<&Ball>,
    cell_query: Query<(Entity, &BoardCellNode)>,
    mut token_query: Query<(&Parent, &mut Visibility), With<PlayerTokenMarker>>,
) {
    let ball = match ball_query.get_single() {
        Ok(b) => b,
        Err(_) => return,
    };

    let active_cell_idx = ball.cell_index as u32;

    for (cell_entity, cell_node) in cell_query.iter() {
        for (parent, mut visibility) in token_query.iter_mut() {
            if parent.get() == cell_entity {
                if cell_node.index == active_cell_idx {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

/// Updates wager token overlays and colors on the 2D board.
pub fn update_wagers_on_board(
    wagers: Res<ClientWagers>,
    cell_query: Query<(Entity, &BoardCellNode)>,
    mut wager_marker_query: Query<(&Parent, &mut Visibility, &mut Sprite, &Children), With<WagerTokenMarker>>,
    mut text_query: Query<&mut Text>,
) {
    for (cell_entity, cell_node) in cell_query.iter() {
        for (parent, mut visibility, mut sprite, children) in wager_marker_query.iter_mut() {
            if parent.get() == cell_entity {
                if let Some(wager) = wagers.0.iter().find(|w| w.cell_index == cell_node.index) {
                    *visibility = Visibility::Visible;
                    sprite.color = match wager.card_type {
                        CardType::Shield => Color::srgb(0.2, 0.4, 0.8),   // Shield (Blue)
                        CardType::Banana => Color::srgb(0.9, 0.8, 0.1),   // Banana (Yellow)
                        CardType::GoldenDie => Color::srgb(0.8, 0.1, 0.1),   // Golden Die (Red)
                    };
                    if let Some(&text_child) = children.first() {
                        if let Ok(mut text) = text_query.get_mut(text_child) {
                            text.sections[0].value = match wager.card_type {
                                CardType::Shield => "S".to_string(),
                                CardType::Banana => "B".to_string(),
                                CardType::GoldenDie => "G".to_string(),
                            };
                        }
                    }
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
