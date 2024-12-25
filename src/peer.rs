use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::num::{NonZero, NonZeroU32};
use std::sync::Arc;
use quinn::{crypto, Endpoint, ServerConfig};
use quinn::rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use rand::random;
use crate::compression::Compression;

pub struct Peer {
    guid: u32,
    addr: SocketAddr,
    server: Endpoint,
    peers: Vec<(NonZero<u32>, Endpoint)>,
}

impl Peer {
    pub async fn serve(addr: SocketAddr, config: PeerConfig) -> Peer {
        // Generate a self-signed TLS certificate for the server
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let cert_der = CertificateDer::from(cert.cert);
        let priv_key = PrivatePkcs8KeyDer::from(cert.key_pair.serialize_der());

        let mut server_config = ServerConfig::with_single_cert(vec![cert_der.clone()], priv_key.into()).unwrap();
        let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
        transport_config.max_concurrent_uni_streams(0_u8.into());

        // Bind the QUIC server to a local address
        let endpoint = Endpoint::server(server_config, addr).unwrap();
        
        Self {
            // The hosts guid will always be zero
            guid: 0,
            addr,
            server: endpoint,
            peers: vec![],
        }
    }

    pub async fn connect(addr: SocketAddr, config: PeerConfig) -> Peer {
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

    pub fn config() -> PeerConfig {
        PeerConfig {
            compress_heartbeat: false,
            compression: (),
            application_name: "".to_string(),
            application_version: 0,
        }
    }
}

pub struct PeerConfig {
    compress_heartbeat: bool,
    compression: Compression,
    application_name: String,
    application_version: u32,
}

impl PeerConfig {
    pub fn compress_heartbeat(mut self, compress_heartbeat: bool) -> Self {
        self.compress_heartbeat = compress_heartbeat;
        self
    }
    
    pub fn compression(mut self, compression: Compression) -> Self {
        self.compression = compression;
        self
    }
    
    pub fn app_name(mut self, application_name: &str) -> Self {
        self.application_name = application_name.to_owned();
        self
    }
    
    pub fn app_version(mut self, app_version: u32) -> Self {
        self.application_version = application_version;
        self
    }
}
