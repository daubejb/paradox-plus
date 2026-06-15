use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct BallVisualInterpolation {
    pub slide_offset: f32, // Progress of visual cell sliding [0.0, 1.0]
}
