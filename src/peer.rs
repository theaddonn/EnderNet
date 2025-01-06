use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use quinn::Endpoint;
use crate::session::{Session, SessionConfig};

pub struct Peer {
    guid: u128,
    endpoint: Endpoint
}

impl Peer {
    pub async fn ping() -> Result<PingPacket> {
        
    }
    
    pub async fn connect(addr: SocketAddr, config: SessionConfig) -> Session {
        let client = Endpoint::client(SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0))).unwrap();

        client

        todo!()
    }

    pub async fn send(&self, data: &[u8], guid: u32) {
        todo!()
    }

    pub async fn send_host(&self, data: &[u8]) {

    }

    pub async fn send_all(&self, data: &[u8]) {

    }

    pub async fn recv(&mut self, data: &mut [u8]) {}
}
