use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PingPacket {
    pub timestamp: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PongPacket {
    pub timestamp: i64,
    pub name: String,
    pub player_max: Option<u32>,
    pub player_count: u32,
}

#[tokio::main]
async fn main() {
    let addr_server = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 5313);
    let addr_client = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 5314);

    let server = endernet::Peer::serve(addr_server, endernet::Peer::config()).await;
    let client = endernet::Peer::connect(addr_server, endernet::Peer::config()).await;

    server.send()

    server.send(&[1, 2, 3, 4]).await;
}
