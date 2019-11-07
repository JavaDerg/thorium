use crate::cbl::ClientBufferList;
use crate::cb::ClientBuffer;
use std::ops::{Deref, DerefMut};
use crate::util::get_time;

pub struct ClientBufferItem {
    pub ip: u128,
    pub id: u32,
    pub buffer: ClientBuffer,
    pub next: Option<ClientBufferItem>,
    pub timeout: u128,
}

impl ClientBufferItem {
    pub fn new(ip: u128, id: u32) -> ClientBufferItem {
        ClientBufferItem {
            ip,
            id,
            buffer: ClientBuffer::new(),
            next: None,
            timeout: get_time(),
        }
    }

    pub fn refresh(&mut self) {
        self.timeout = get_time();
    }

    pub fn is_overdue(&self) -> bool {
        self.is_overdue_by_amount(30000)
    }

    pub fn is_overdue_by_amount(&self, overdue_ms: u32) -> bool {
        //TODO: actualy implement this!
        return false;
    }
}

impl Deref for ClientBufferItem {
    type Target = ClientBuffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for ClientBufferItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}