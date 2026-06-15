use bevy::prelude::*;
use heapless::Vec as HVec;
use protocol::messages::{MAX_PLAYERS, CardType};
use protocol::physics::MovementDirection;
use fixed::types::I32F32;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use bevy::tasks::Task;
use crate::ai::mdp_solver::MdpSolverTable;

#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct CourseTrackResource(pub protocol::terrain::ActiveCourseTrack);

#[derive(Resource, Clone, PartialEq, Eq, Debug)]
pub struct StructuralEpoch {
    pub player_positions: HVec<u16, MAX_PLAYERS>,
    pub epoch_id: u32,
}

impl Default for StructuralEpoch {
    fn default() -> Self {
        Self {
            player_positions: HVec::new(),
            epoch_id: 0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub player_id: u64,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ball {
    pub cell_index: u16,
    pub direction: MovementDirection,
    pub origin_cell: u16,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bot {
    pub difficulty_threshold: I32F32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveBotTurn;

#[derive(Component)]
pub struct ActiveMdpSolverTask {
    pub task: Option<Task<Option<MdpSolverTable>>>,
    pub cancel_flag: Arc<AtomicBool>,
    pub structural_epoch: StructuralEpoch,
}

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct TurnTimer {
    pub elapsed: I32F32,
}

#[derive(Resource)]
pub struct DeterministicRng(pub ChaCha8Rng);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WagerToken {
    pub card_type: CardType,
    pub owner_id: u64,
    pub cell_index: u16,
}
