mod phc;

use crate::cbl::ClientBufferList;
use std::net::{UdpSocket, SocketAddr};
use crate::ph::phc::ProxyHostConfiguration;

struct ProxyHost {
    client_buffers: ClientBufferList
}

impl ProxyHost {
    pub fn new(config: ProxyHostConfiguration) -> ClientBufferList {
        UdpSocket::bind()
    }
}