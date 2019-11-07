use std::ops::{Deref, DerefMut};

const BUFFER_SIZE: usize = 64512;

pub struct ClientBuffer(pub [u8; BUFFER_SIZE]);

impl ClientBuffer {
    pub fn new() -> ClientBuffer {
        [u8; 0..BUFFER_SIZE]
    }

    pub fn clear(&mut self) {
        self.0 = [u8; 0..BUFFER_SIZE];
    }
}

impl Deref for ClientBuffer {
    type Target = [u8; BUFFER_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClientBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}