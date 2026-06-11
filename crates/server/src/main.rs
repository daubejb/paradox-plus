use bevy::prelude::*;
use std::net::SocketAddr;
use tokio::sync::mpsc::channel;

use server::systems::{
    ServerGameState, fsm_tick_system, validate_actions_system, ServerActionReceiver,
    NetworkSerializationBuffer, ClientActionEvent, ClientActionMessage,
    broadcast_state_sync_system, ServerActionSender
};

fn main() {
    let (_action_tx, action_rx) = channel::<ClientActionMessage>(100);
    let (broadcast_tx, _broadcast_rx) = channel::<(SocketAddr, Vec<u8>)>(100);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    // Spawn async listener
    runtime.spawn(async move {
        let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        if let Ok(config) = configure_dummy_quic() {
            let _ = quinn::Endpoint::server(config, addr);
        }
    });

    let mut app = App::new();
    app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_millis(16)))
        .insert_resource(ServerGameState::default())
        .insert_resource(ServerActionReceiver { rx: action_rx })
        .insert_resource(ServerActionSender {
            tx: broadcast_tx,
            clients: vec![],
        })
        .insert_resource(NetworkSerializationBuffer::default())
        .add_event::<ClientActionEvent>()
        .add_systems(Update, (
            validate_actions_system,
            fsm_tick_system,
            broadcast_state_sync_system,
        ).chain());

    println!("Server running!");
    app.run();
}

fn configure_dummy_quic() -> Result<quinn::ServerConfig, Box<dyn std::error::Error>> {
    Err("TLS certs not configured".into())
}
