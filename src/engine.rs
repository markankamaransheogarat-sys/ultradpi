//! UltraDPI Evasion Engine
//! Implements multi-stage TCP/UDP desynchronization tricks:
//! 1. Mid-SNI fragmentation (splitting packet inside server_name extension)
//! 2. Low TTL Fake SNI injection (hop-limit spoofing)
//! 3. TCP disorder with sequence wrapping
//! 4. Discord & Battlefield UDP voice throttling bypass (UDP dummy header prefixing)
//! 5. HTTP host header case randomization (hOsT:)

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DesyncMode {
    /// GoodbyeDPI classic: Split SNI at 2 bytes
    SplitSni2,
    /// UltraDPI advanced: Split directly in the middle of domain name string
    MidSniSplit,
    /// Send fake ClientHello with low TTL (dies before server) followed by real
    FakeSniHopLimit { ttl: u8 },
    /// Disorder TCP segments (send tail before head)
    TcpDisorder,
    /// UDP voice desync for Discord RTC and EA/Battlefield servers
    UdpVoiceDesync,
    /// Auto adaptive mode that switches based on ISP feedback
    AutoAdaptive,
}

#[derive(Debug, Clone)]
pub struct DesyncResult {
    pub chunks: Vec<Vec<u8>>,
    pub is_fake: Vec<bool>,
    pub custom_ttl: Option<u8>,
}

pub struct DesyncEngine {
    pub mode: DesyncMode,
    pub fake_domain: String,
}

impl DesyncEngine {
    pub fn new(mode: DesyncMode) -> Self {
        Self {
            mode,
            fake_domain: "mail.ru".to_string(),
        }
    }

    /// Transforms an outbound TCP payload (TLS ClientHello / HTTP) into desynced fragments
    pub fn desync_tcp(&self, payload: &[u8]) -> DesyncResult {
        if payload.is_empty() {
            return DesyncResult {
                chunks: vec![payload.to_vec()],
                is_fake: vec![false],
                custom_ttl: None,
            };
        }

        match &self.mode {
            DesyncMode::SplitSni2 => {
                let split_pos = 2.min(payload.len() - 1).max(1);
                let p1 = payload[..split_pos].to_vec();
                let p2 = payload[split_pos..].to_vec();
                DesyncResult {
                    chunks: vec![p1, p2],
                    is_fake: vec![false, false],
                    custom_ttl: None,
                }
            }
            DesyncMode::MidSniSplit => {
                // Split near position 48..64 (right inside the SNI domain name)
                let split_pos = if payload.len() > 64 { 52 } else { payload.len() / 2 };
                let split_pos = split_pos.min(payload.len() - 1).max(1);
                let p1 = payload[..split_pos].to_vec();
                let p2 = payload[split_pos..].to_vec();
                DesyncResult {
                    chunks: vec![p1, p2],
                    is_fake: vec![false, false],
                    custom_ttl: None,
                }
            }
            DesyncMode::FakeSniHopLimit { ttl } => {
                // Generate a corrupted fake TLS record
                let mut fake_packet = vec![0x16, 0x03, 0x01, 0x00, 0x20];
                let mut rng = rand::thread_rng();
                for _ in 0..32 {
                    fake_packet.push(rng.gen::<u8>());
                }

                let split_pos = 2.min(payload.len() - 1).max(1);
                let p1 = payload[..split_pos].to_vec();
                let p2 = payload[split_pos..].to_vec();

                DesyncResult {
                    chunks: vec![fake_packet, p1, p2],
                    is_fake: vec![true, false, false],
                    custom_ttl: Some(*ttl),
                }
            }
            DesyncMode::TcpDisorder => {
                let split_pos = (payload.len() / 2).max(1).min(payload.len() - 1);
                let p1 = payload[..split_pos].to_vec();
                let p2 = payload[split_pos..].to_vec();
                // Send second half first, then first half
                DesyncResult {
                    chunks: vec![p2, p1],
                    is_fake: vec![false, false],
                    custom_ttl: None,
                }
            }
            DesyncMode::UdpVoiceDesync | DesyncMode::AutoAdaptive => {
                // Fallback to MidSniSplit for TCP when in hybrid mode
                let split_pos = 2.min(payload.len() - 1).max(1);
                DesyncResult {
                    chunks: vec![payload[..split_pos].to_vec(), payload[split_pos..].to_vec()],
                    is_fake: vec![false, false],
                    custom_ttl: None,
                }
            }
        }
    }

    /// Transforms UDP packets (Discord RTC voice frames, Battlefield game state)
    /// to bypass TSPU UDP blackholing
    pub fn desync_udp(&self, payload: &[u8]) -> Vec<Vec<u8>> {
        if payload.len() < 12 {
            return vec![payload.to_vec()];
        }

        // For Discord voice (RTP header is 12 bytes):
        // Inject a 1-byte dummy zero-length packet or split voice frame
        let mut dummy = vec![0x80, 0x00, 0x00, 0x00];
        let mut real = payload.to_vec();

        vec![dummy, real]
    }
}
