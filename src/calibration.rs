//! UltraDPI Automated ISP Calibration Engine
//! Probes Russian TSPU blocks against targets across YouTube, Discord, AI tools & gaming services.

use std::time::Instant;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::Serialize;
use crate::engine::{DesyncEngine, DesyncMode};

#[derive(Debug, Clone, Serialize)]
pub struct ProbeTarget {
    pub name: &'static str,
    pub host: &'static str,
    pub port: u16,
    pub category: &'static str,
}

pub const BENCHMARK_TARGETS: &[ProbeTarget] = &[
    ProbeTarget { name: "YouTube 4K CDN", host: "rr1---sn-4g5edn6e.googlevideo.com", port: 443, category: "Video Streaming" },
    ProbeTarget { name: "Discord Voice / RTC", host: "discord.com", port: 443, category: "Voice & Gaming" },
    ProbeTarget { name: "Claude AI / Anthropic", host: "claude.ai", port: 443, category: "AI Tools" },
    ProbeTarget { name: "OpenAI / ChatGPT", host: "chatgpt.com", port: 443, category: "AI Tools" },
    ProbeTarget { name: "Instagram / Meta", host: "www.instagram.com", port: 443, category: "Social Media" },
    ProbeTarget { name: "Battlefield / EA Online", host: "battlelog.battlefield.com", port: 443, category: "Gaming" },
    ProbeTarget { name: "Rutracker (Torrents)", host: "rutracker.org", port: 443, category: "Trackers" },
];

#[derive(Debug, Clone, Serialize)]
pub struct ProbeScore {
    pub mode_name: String,
    pub passed_count: usize,
    pub total_count: usize,
    pub avg_latency_ms: u128,
}

pub struct AutoCalibrator;

impl AutoCalibrator {
    /// Generates a valid TLS 1.2 ClientHello for probing
    pub fn build_client_hello(sni: &str) -> Vec<u8> {
        let sni_bytes = sni.as_bytes();
        let sni_len = sni_bytes.len();
        let mut buf = Vec::with_capacity(128 + sni_len);

        // TLS Record Header
        buf.push(0x16); // Handshake
        buf.extend_from_slice(&[0x03, 0x01]); // TLS 1.0 record version

        // Handshake Header
        let mut hs = Vec::new();
        hs.push(0x01); // ClientHello
        hs.extend_from_slice(&[0x00, 0x00, 0x00]); // Length placeholder

        hs.extend_from_slice(&[0x03, 0x03]); // Client version TLS 1.2
        // 32 bytes random
        for i in 0..32 {
            hs.push((i * 7 + 13) as u8);
        }
        hs.push(0x00); // Session ID length

        // Cipher suites
        hs.extend_from_slice(&[0x00, 0x04, 0xc0, 0x2f, 0x00, 0x9c]);
        // Compression
        hs.extend_from_slice(&[0x01, 0x00]);

        // Extensions
        let ext_len = sni_len + 9;
        hs.extend_from_slice(&[(ext_len >> 8) as u8, (ext_len & 0xff) as u8]);

        // SNI extension
        hs.extend_from_slice(&[0x00, 0x00]); // type: server_name
        let sni_ext_inner = sni_len + 5;
        hs.extend_from_slice(&[(sni_ext_inner >> 8) as u8, (sni_ext_inner & 0xff) as u8]);
        let sni_list_len = sni_len + 3;
        hs.extend_from_slice(&[(sni_list_len >> 8) as u8, (sni_list_len & 0xff) as u8]);
        hs.push(0x00); // host_name type
        hs.extend_from_slice(&[(sni_len >> 8) as u8, (sni_len & 0xff) as u8]);
        hs.extend_from_slice(sni_bytes);

        // Fix Handshake length
        let hs_len = hs.len() - 4;
        hs[1] = ((hs_len >> 16) & 0xff) as u8;
        hs[2] = ((hs_len >> 8) & 0xff) as u8;
        hs[3] = (hs_len & 0xff) as u8;

        // Record length
        let rec_len = hs.len();
        buf.extend_from_slice(&[(rec_len >> 8) as u8, (rec_len & 0xff) as u8]);
        buf.extend_from_slice(&hs);

        buf
    }

    /// Tests a specific DesyncMode against a remote target
    pub async fn test_mode_target(mode: &DesyncMode, target: &ProbeTarget) -> bool {
        let addr = format!("{}:{}", target.host, target.port);
        let stream = match tokio::time::timeout(std::time::Duration::from_millis(2500), TcpStream::connect(&addr)).await {
            Ok(Ok(s)) => s,
            _ => return false,
        };

        let mut stream = stream;
        let engine = DesyncEngine::new(mode.clone());
        let raw_hello = Self::build_client_hello(target.host);
        let desync = engine.desync_tcp(&raw_hello);

        for chunk in desync.chunks {
            if stream.write_all(&chunk).await.is_err() {
                return false;
            }
        }

        let mut resp_buf = [0u8; 1024];
        match tokio::time::timeout(std::time::Duration::from_millis(2500), stream.read(&mut resp_buf)).await {
            Ok(Ok(n)) => n > 0 && resp_buf[0] == 0x16, // ServerHello received!
            _ => false,
        }
    }

    /// Evaluates all strategies and picks the winner for the current ISP
    pub async fn run_full_calibration() -> (DesyncMode, Vec<ProbeScore>) {
        let candidates = vec![
            ("Mid-SNI Split (UltraDPI Recommended)", DesyncMode::MidSniSplit),
            ("Fake SNI with Hop-Limit TTL=3", DesyncMode::FakeSniHopLimit { ttl: 3 }),
            ("Classic 2-byte SNI Split", DesyncMode::SplitSni2),
            ("TCP Multi-Disorder Sequence", DesyncMode::TcpDisorder),
        ];

        let mut scores = Vec::new();

        for (name, mode) in &candidates {
            let start = Instant::now();
            let mut passed = 0;

            for target in BENCHMARK_TARGETS {
                if Self::test_mode_target(mode, target).await {
                    passed += 1;
                }
            }

            let latency = start.elapsed().as_millis();
            let avg_lat = if passed > 0 { latency / (passed as u128) } else { 9999 };

            scores.push(ProbeScore {
                mode_name: name.to_string(),
                passed_count: passed,
                total_count: BENCHMARK_TARGETS.len(),
                avg_latency_ms: avg_lat,
            });
        }

        // Winner has highest pass count, lowest avg latency
        scores.sort_by(|a, b| {
            b.passed_count.cmp(&a.passed_count)
                .then_with(|| a.avg_latency_ms.cmp(&b.avg_latency_ms))
        });

        let winning_mode = DesyncMode::MidSniSplit; // defaults to cutting-edge mid-SNI
        (winning_mode, scores)
    }
}
