use bevy::prelude::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;

pub const MAX_PACKET_SIZE: usize = 65536;

#[derive(Resource)]
pub struct NetworkSerializationBuffer {
    /// Bounded buffer boxed on heap to prevent stack overflow.
    pub buffer: Box<heapless::Vec<u8, MAX_PACKET_SIZE>>,
}

impl Default for NetworkSerializationBuffer {
    fn default() -> Self {
        Self {
            buffer: Box::new(heapless::Vec::new()),
        }
    }
}

#[derive(Resource)]
pub struct ClientActionSender {
    pub sender: Sender<Vec<u8>>,
}

#[derive(Resource)]
pub struct ServerUpdateReceiver {
    pub receiver: Mutex<Receiver<Vec<u8>>>,
}
