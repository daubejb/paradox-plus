use bevy::prelude::*;
use client::{
    init_client_environment,
    network::{ClientNetworkPlugin, ClientActionSender, ServerUpdateReceiver},
    replication::ClientReplicationPlugin,
    presenter::FixedToFloatPlugin,
    ui::ClientUiPlugin,
};

fn main() {
    init_client_environment();

    let mut app = App::new();

    // Add standard Bevy DefaultPlugins to open a rendering window
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Paradox Plus".to_string(),
            resolution: (450.0, 800.0).into(), // Mobile portrait ratio matching the mockup
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
    app.insert_resource(client::ui::systems::simulation::LocalServerChannels {
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

    // Run the native event loop
    app.run();
}
