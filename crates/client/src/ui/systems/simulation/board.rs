use bevy::prelude::*;
use crate::replication::Ball;
use crate::ui::components::{BoardCellNode, PlayerTokenMarker};

pub fn update_board_cell_visuals(
    ball_query: Query<&Ball, Changed<Ball>>,
    cell_query: Query<(Entity, &BoardCellNode)>,
    mut token_query: Query<(&Parent, &mut Style), With<PlayerTokenMarker>>,
) {
    let ball = match ball_query.get_single() {
        Ok(b) => b,
        Err(_) => return,
    };

    let active_cell_idx = ball.cell_index as u32;

    for (cell_entity, cell_node) in cell_query.iter() {
        for (parent, mut style) in token_query.iter_mut() {
            if parent.get() == cell_entity {
                if cell_node.index == active_cell_idx {
                    style.display = Display::Flex;
                } else {
                    style.display = Display::None;
                }
            }
        }
    }
}
