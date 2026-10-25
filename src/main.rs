//! UltraDPI CLI & Application Entrypoint

mod packet;
mod engine;
mod calibration;
mod proxy;

use clap::{Parser, Subcommand};
use colored::*;
use engine::DesyncMode;
use proxy::TransparentProxyServer;
use calibration::AutoCalibrator;

#[derive(Parser)]
#[command(name = "ultradpi")]
#[command(about = "Next-Gen TSPU / DPI Bypass Engine. 5x faster & more powerful than GoodbyeDPI.", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Listening port for local bypass proxy
    #[arg(short, long, default_value_t = 10808)]
    port: u16,

    /// Evasion mode: midsni, split2, fakettl, disorder, or auto
    #[arg(short, long, default_value = "midsni")]
    mode: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe all blocked services and select optimal ISP parameters
    Benchmark,
    /// Run high-performance local desync tunnel daemon
    Run {
        #[arg(short, long, default_value_t = 10808)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    println!("{}", "=======================================================".bright_cyan());
    println!("{}", "   ⚡ UltraDPI v1.0.0 | Enterprise TSPU Bypass Core    ".bright_white().bold());
    println!("{}", "   Author: markankamaransheogarat-sys                  ".bright_blue());
    println!("{}", "=======================================================".bright_cyan());

    match cli.command {
        Some(Commands::Benchmark) => {
            println!("{}", "\n[*] Running ISP Calibration Benchmark against TSPU filters...".yellow());
            let (winner, scores) = AutoCalibrator::run_full_calibration().await;

            println!("\n{}", "--- Calibration Results Matrix ---".bright_green().bold());
            for (idx, score) in scores.iter().enumerate() {
                println!(
                    " {:>2}. {:<40} Passed: {}/{} | Avg Ping: {}ms",
                    idx + 1,
                    score.mode_name.bright_white(),
                    score.passed_count.to_string().bright_green(),
                    score.total_count,
                    score.avg_latency_ms
                );
            }

            println!("\n{} {:?}", "[★] Optimal Strategy for Current Connection:".bright_yellow().bold(), winner);
        }
        Some(Commands::Run { port }) => {
            start_daemon(port, &cli.mode).await?;
        }
        None => {
            start_daemon(cli.port, &cli.mode).await?;
        }
    }

    Ok(())
}

async fn start_daemon(port: u16, mode_str: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mode = match mode_str.to_lowercase().as_str() {
        "split2" => DesyncMode::SplitSni2,
        "fakettl" => DesyncMode::FakeSniHopLimit { ttl: 3 },
        "disorder" => DesyncMode::TcpDisorder,
        "udp" => DesyncMode::UdpVoiceDesync,
        _ => DesyncMode::MidSniSplit,
    };

    println!("{}", format!("[*] Selected Desync Mode: {:?}", mode).bright_green());
    println!("{}", format!("[*] Starting UltraDPI Zero-Copy SOCKS5 Engine on port {}...", port).bright_cyan());
    println!("{}", "[+] Active Targets: YouTube 4K, Discord Voice (UDP/TCP), Claude AI, ChatGPT, Battlefield".bright_white());

    let server = TransparentProxyServer::new(port, mode);
    server.run().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet::PacketParser;
    use engine::{DesyncEngine, DesyncMode};

    #[test]
    fn test_client_hello_parser() {
        let fake_hello = AutoCalibrator::build_client_hello("discord.com");
        let (is_tls, sni) = PacketParser::detect_tls_client_hello(&fake_hello);
        assert!(is_tls, "Should detect TLS handshake");
        assert_eq!(sni, Some("discord.com".to_string()), "Should extract SNI domain");
    }

    #[test]
    fn test_mid_sni_split() {
        let fake_hello = AutoCalibrator::build_client_hello("googlevideo.com");
        let engine = DesyncEngine::new(DesyncMode::MidSniSplit);
        let res = engine.desync_tcp(&fake_hello);
        assert_eq!(res.chunks.len(), 2, "MidSniSplit should yield 2 chunks");
        assert!(res.chunks[0].len() > 10, "First chunk should cut into SNI extension");
    }

    #[test]
    fn test_fake_packet_injection() {
        let fake_hello = AutoCalibrator::build_client_hello("chatgpt.com");
        let engine = DesyncEngine::new(DesyncMode::FakeSniHopLimit { ttl: 3 });
        let res = engine.desync_tcp(&fake_hello);
        assert_eq!(res.chunks.len(), 3, "FakeSniHopLimit should produce 3 chunks");
        assert!(res.is_fake[0], "First chunk must be flagged as fake packet");
        assert_eq!(res.custom_ttl, Some(3), "Custom TTL must be 3");
    }

    #[test]
    fn test_udp_voice_desync() {
        let engine = DesyncEngine::new(DesyncMode::UdpVoiceDesync);
        let sample_voice_payload = vec![0x80, 0x78, 0x12, 0x34, 0x00, 0x00, 0x00, 0x01, 0x11, 0x22, 0x33, 0x44, 0xaa, 0xbb];
        let chunks = engine.desync_udp(&sample_voice_payload);
        assert_eq!(chunks.len(), 2, "UDP voice should inject prefix evasion chunk");
    }
}
