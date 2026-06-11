use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub mod mdp_state;
pub mod mdp_solver;
pub mod components;
pub mod systems {
    pub mod solver;
    pub mod takeover;
}

pub use mdp_state::MdpState;
pub use components::{
    StructuralEpoch, Bot, ActiveBotTurn, ActiveMdpSolverTask, TurnTimer, DeterministicRng, Player, Ball, WagerToken, CourseTrackResource
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StructuralEpoch::default())
            .insert_resource(TurnTimer::default())
            .insert_resource(DeterministicRng(ChaCha8Rng::seed_from_u64(1337)))
            .add_systems(Update, (
                systems::solver::trigger_ai_solver_system,
                systems::solver::poll_ai_solver_system,
                systems::takeover::turn_timeout_takeover_system,
            ).chain());
    }
}
