pub mod loopback;
pub mod board;
pub mod render;

pub use loopback::{local_offline_server_system, LocalServerChannels, trigger_initial_state_sync};
pub use loopback::state::OfflineServerState;
pub use board::{update_board_cell_visuals, rebuild_board_on_hole_change_system};
pub use render::update_ui_elements_system;
