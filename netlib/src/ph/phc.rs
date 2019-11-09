use std::net::{SocketAddr, ToSocketAddrs};
use std::thread::AccessError;

pub struct ProxyHostConfiguration {
    pub targets: Vec<BindTarget>
}

pub struct BindTarget {
    pub address: String,
    pub port: u16,
}

impl BindTarget {
    pub fn new(address: String, port: u16) -> BindTarget {
        BindTarget {
            address,
            port,
        }
    }
}