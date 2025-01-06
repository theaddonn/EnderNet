use std::borrow::Cow;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
use std::num::{NonZero, NonZeroU32};
use std::sync::Arc;
use quinn::{crypto, Endpoint, ServerConfig};
use quinn::rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use rand::random;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::compression::Compression;
use crate::peer::Peer;

pub struct Session<'de, PingPacket, PongPacket>
where
    PingPacket: Deserialize<'de>
{
    addr: SocketAddr,
    server: Endpoint,
    peers: Vec<Peer>,
    session_config: ServerConfig,
    pong_state: Arc<RwLock<PongPacket>>,
}

impl<'de, PingPacket> Session<'de, PingPacket>
where
    PingPacket: Serialize + Deserialize<'de>
{
    pub async fn serve(addr: SocketAddr, config: SessionConfig) -> Self {
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
            addr,
            server: endpoint,
            peers: vec![],
            session_config: config,
        }
    }

    
}

pub struct SessionConfig {
    compress_heartbeat: bool,
    compression: Compression,
    app_name: String,
    app_version: semver::Version,
    app_version_req: Option<semver::VersionReq>,
}

impl SessionConfig {
    pub fn new(name: impl AsRef<str>, version: semver::Version) -> Self {
        Self {
            compress_heartbeat: false,
            compression: Compression::new(),
            app_name: name.as_ref().to_string(),
            app_version: version,
        }
    }

    pub fn compress_heartbeat(mut self, compress_heartbeat: bool) -> Self {
        self.compress_heartbeat = compress_heartbeat;
        self
    }
    
    pub fn compression(mut self, compression: Compression) -> Self {
        self.compression = compression;
        self
    }
    
    pub fn app_version_req(mut self, semver: semver::VersionReq) -> Self {
        self.app_version_req = Some(semver);
        self
    }
}
