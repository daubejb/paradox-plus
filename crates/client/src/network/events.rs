use bevy::prelude::*;
use protocol::messages::{ClientAction, ServerUpdate};

#[derive(Event, Debug, Clone)]
pub struct ServerUpdateEvent(pub ServerUpdate);

#[derive(Event, Debug, Clone)]
pub struct ClientActionRequest(pub ClientAction);
