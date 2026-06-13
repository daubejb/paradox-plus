use bevy::prelude::*;
use protocol::messages::{GameStateEnum, ServerUpdate};
use protocol::physics::MovementDirection;
use crate::network::ServerUpdateEvent;
use crate::presenter::components::BallVisualInterpolation;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ClientGameState {
    #[default]
    Lobby,
    MarkerPlacement,
    AwaitingTurn,
    Rolling,
    Moving,
    BananaChoice,
    HazardAlert,
    HoleCompleted,
    MatchCompleted,
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

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePlayerId(pub u64);

pub fn map_enum_to_bevy_state(state: GameStateEnum) -> ClientGameState {
    match state {
        GameStateEnum::Lobby => ClientGameState::Lobby,
        GameStateEnum::MarkerPlacement => ClientGameState::MarkerPlacement,
        GameStateEnum::AwaitingTurn => ClientGameState::AwaitingTurn,
        GameStateEnum::Rolling => ClientGameState::Rolling,
        GameStateEnum::Moving => ClientGameState::Moving,
        GameStateEnum::BananaChoice => ClientGameState::BananaChoice,
        GameStateEnum::HazardAlert => ClientGameState::HazardAlert,
        GameStateEnum::HoleCompleted => ClientGameState::HoleCompleted,
        GameStateEnum::MatchCompleted => ClientGameState::MatchCompleted,
    }
}

pub fn sync_state_from_server(
    mut commands: Commands,
    mut update_events: EventReader<ServerUpdateEvent>,
    mut next_state: ResMut<NextState<ClientGameState>>,
    mut current_hole: ResMut<crate::ui::components::CurrentHole>,
    mut client_wagers: ResMut<crate::ui::components::ClientWagers>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in update_events.read() {
        if let ServerUpdate::StateSync {
            game_state,
            active_player_id,
            player_positions,
            current_hole: sync_hole,
            placed_wagers,
            ..
        } = &event.0
        {
            // Update ClientWagers resource
            client_wagers.0 = placed_wagers.to_vec();

            // Guard assignment to preserve Bevy's change detection
            if current_hole.0 != *sync_hole {
                current_hole.0 = *sync_hole;
            }

            commands.insert_resource(ActivePlayerId(*active_player_id));

            // Update ClientGameState
            let target_state = map_enum_to_bevy_state(*game_state);
            next_state.set(target_state);

            // Replicate player and ball entities: despawn old, spawn new
            for entity in player_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            for (i, pos) in player_positions.iter().enumerate() {
                let player_id = if i == 0 { *active_player_id } else { i as u64 };
                commands.spawn((
                    Player { player_id },
                    Ball {
                        cell_index: *pos as u16,
                        direction: MovementDirection::Forward,
                        origin_cell: *pos as u16,
                    },
                    BallVisualInterpolation { slide_offset: 0.0 },
                    Transform::default(),
                    GlobalTransform::default(),
                ));
            }
        }
    }
}

pub struct ClientReplicationPlugin;

impl Plugin for ClientReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientGameState>()
            .init_resource::<crate::ui::components::CurrentHole>()
            .init_resource::<crate::ui::components::ClientWagers>()
            .init_resource::<ActivePlayerId>()
            .add_systems(Update, sync_state_from_server);
    }
}
