use bevy::prelude::*;
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

/// Updates wager token overlays and visibility on the 2D board.
pub fn update_wagers_on_board(
    wagers: Res<ClientWagers>,
    cell_query: Query<(Entity, &BoardCellNode)>,
    mut wager_marker_query: Query<(Entity, &Parent, &mut Visibility), With<WagerTokenMarker>>,
    mut wager_visual_query: Query<(&Parent, &super::token::WagerVisual, &mut Visibility), Without<WagerTokenMarker>>,
) {
    for (cell_entity, cell_node) in cell_query.iter() {
        for (marker_entity, parent, mut root_visibility) in wager_marker_query.iter_mut() {
            if parent.get() == cell_entity {
                if let Some(wager) = wagers.0.iter().find(|w| w.cell_index == cell_node.index) {
                    let target_root_vis = Visibility::Inherited;
                    if *root_visibility != target_root_vis {
                        *root_visibility = target_root_vis;
                    }

                    for (visual_parent, visual, mut visual_visibility) in wager_visual_query.iter_mut() {
                        if visual_parent.get() == marker_entity {
                            let target_visual_vis = if visual.card_type == wager.card_type {
                                Visibility::Inherited
                            } else {
                                Visibility::Hidden
                            };

                            if *visual_visibility != target_visual_vis {
                                *visual_visibility = target_visual_vis;
                            }
                        }
                    }
                } else {
                    let target_root_vis = Visibility::Hidden;
                    if *root_visibility != target_root_vis {
                        *root_visibility = target_root_vis;
                    }
                }
            }
        }
    }
}

