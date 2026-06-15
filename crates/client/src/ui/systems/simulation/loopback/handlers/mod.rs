pub mod dice;
pub mod terrain;
pub mod movement;
pub mod wager;
pub mod banana;
pub mod roll;

use super::state::OfflineServerState;
use protocol::messages::{ClientAction, ServerUpdate};
use protocol::terrain::ActiveCourseTrack;

pub fn handle_action(
    state: &mut OfflineServerState,
    action: &ClientAction,
    course: &ActiveCourseTrack,
) -> Vec<ServerUpdate> {
    let mut updates = Vec::new();

    match action {
        ClientAction::RollDice { dice_count } => {
            updates.extend(roll::handle_roll_dice(state, *dice_count, course));
        }
        ClientAction::DraftCard { card_type, cell_index } => {
            updates.extend(wager::handle_draft_card(state, *card_type, *cell_index, course));
        }
        ClientAction::SkipPlacement => {
            updates.extend(wager::handle_skip_placement(state));
        }
        ClientAction::ChooseBananaSlide { step_count } => {
            updates.extend(banana::handle_choose_banana_slide(state, *step_count, course));
        }
        _ => {}
    }

    updates
}
