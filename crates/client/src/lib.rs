use bevy::prelude::*;
use network::{ClientNetworkPlugin, ClientActionSender, ServerUpdateReceiver};
use replication::ClientReplicationPlugin;
use presenter::FixedToFloatPlugin;
use ui::ClientUiPlugin;

pub mod network;
pub mod replication;
pub mod presenter;
pub mod ui;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod mobile;

pub fn init_client_environment() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }
}

pub fn setup_client_app(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Paradox Plus".to_string(),
            resolution: (450.0, 800.0).into(), // Mobile portrait ratio
            ..default()
        }),
        ..default()
    }));

    // Setup loopback network channels for local offline mode
    let (action_tx, action_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (update_tx, update_rx) = std::sync::mpsc::channel::<Vec<u8>>();

    app.insert_resource(ServerUpdateReceiver {
        receiver: std::sync::Mutex::new(update_rx),
    });
    app.insert_resource(ClientActionSender {
        sender: action_tx,
    });
    app.insert_resource(ui::systems::simulation::LocalServerChannels {
        action_rx: std::sync::Mutex::new(action_rx),
        update_tx,
        send_buf: std::sync::Mutex::new(Vec::with_capacity(65536)),
    });

    // Register our game plugins
    app.add_plugins((
        ClientNetworkPlugin,
        ClientReplicationPlugin,
        FixedToFloatPlugin,
        ClientUiPlugin,
    ));
}

#[cfg(target_os = "android")]
pub fn run_client(android_app: bevy::winit::AndroidApp) {
    let mut app = App::new();
    app.insert_non_send_resource(android_app);
    setup_client_app(&mut app);
    app.run();
}

#[cfg(not(target_os = "android"))]
pub fn run_client() {
    let mut app = App::new();
    setup_client_app(&mut app);
    app.run();
}
