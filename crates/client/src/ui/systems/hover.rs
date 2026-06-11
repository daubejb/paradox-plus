use bevy::prelude::*;

const HOVER_SCALE: f32 = 1.05;
const DEFAULT_SCALE: f32 = 1.0;

/// Handles smooth button scaling and z-indexing on hover.
pub fn handle_button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut Transform, &mut ZIndex),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut transform, mut z_index) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                transform.scale = Vec3::splat(HOVER_SCALE);
                *z_index = ZIndex::Local(10);
            }
            Interaction::None => {
                transform.scale = Vec3::splat(DEFAULT_SCALE);
                *z_index = ZIndex::Local(0);
            }
            Interaction::Pressed => {
                transform.scale = Vec3::splat(DEFAULT_SCALE);
            }
        }
    }
}
