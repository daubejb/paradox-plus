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

    // Setup dummy/mock network channels to prevent systems from panicking
    let (_server_tx, client_rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let (client_tx, _server_rx) = std::sync::mpsc::channel::<Vec<u8>>();

    app.insert_resource(ServerUpdateReceiver {
        receiver: std::sync::Mutex::new(client_rx),
    });
    app.insert_resource(ClientActionSender {
        sender: client_tx,
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
