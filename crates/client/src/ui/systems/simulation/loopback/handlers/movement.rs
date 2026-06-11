use protocol::physics::MovementDirection;
use protocol::terrain::{ActiveCourseTrack, TerrainType};

pub fn resolve_movement_position(
    current_position: u32,
    roll: u8,
    direction: MovementDirection,
    course: &ActiveCourseTrack,
) -> (u32, MovementDirection) {
    let total_cells = course.total_cells;

    // Scan course to find Green range (min_green_idx and max_green_idx)
    let mut max_green_idx = 0u32;
    let mut min_green_idx = total_cells;
    for (i, cell_type) in course.cells.iter().enumerate() {
        if let TerrainType::Green(_) = cell_type {
            let idx = i as u32;
            if idx > max_green_idx {
                max_green_idx = idx;
            }
            if idx < min_green_idx {
                min_green_idx = idx;
            }
        }
    }

    let mut target_cell;
    let mut next_direction = direction;

    match direction {
        MovementDirection::Forward => {
            target_cell = current_position.saturating_add(roll as u32);
            if target_cell > max_green_idx {
                next_direction = MovementDirection::Reverse;
            }
            if target_cell >= total_cells {
                target_cell = total_cells.saturating_sub(1);
            }
        }
        MovementDirection::Reverse => {
            if current_position < roll as u32 {
                target_cell = 1;
                next_direction = MovementDirection::Forward;
            } else {
                target_cell = current_position.saturating_sub(roll as u32);
                if target_cell < min_green_idx {
                    next_direction = MovementDirection::Forward;
                }
                if target_cell < 1 {
                    target_cell = 1;
                    next_direction = MovementDirection::Forward;
                }
            }
        }
    }

    (target_cell, next_direction)
}
