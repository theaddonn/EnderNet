use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::time::Duration;
use quinn::Endpoint;
use serde::Deserialize;
use crate::session::{Session, SessionConfig};

pub struct Peer<'de, PingPacket, PongPacket>
where
    PingPacket: Deserialize<'de> {
    id: PeerID,
    endpoint: Endpoint
}

#[derive(Debug, Copy, Clone, Eq)]
pub struct PeerID(u32);

impl<'de, PingPacket, PongPacket> Peer<'de, PingPacket, PongPacket> where
    PingPacket: Deserialize<'de>  {
    pub async fn ping() -> Result<(PingPacket, Duration)> {
        todo!()
    }
    
    pub async fn connect(addr: SocketAddr, config: SessionConfig) -> Session<'de, PingPacket, PongPacket> {
        let client = Endpoint::client(SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0))).unwrap();

        todo!()
    }

    pub fn id(&self) -> PeerID {
        self.id
    }
    
    pub async fn send(&self, data: &[u8], id: PeerID) {
        todo!()
    }

    pub async fn send_host(&self, data: &[u8]) {

    }

    pub async fn send_all(&self, data: &[u8]) {

    }

    pub async fn recv(&mut self, data: &mut [u8]) {}
}
