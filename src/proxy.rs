//! UltraDPI High-Performance Zero-Copy SOCKS5 & Transparent Proxy Server

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use crate::engine::{DesyncEngine, DesyncMode};

pub struct ProxyStats {
    pub active_conns: AtomicUsize,
    pub total_conns: AtomicUsize,
    pub bytes_processed: AtomicU64,
}

pub struct TransparentProxyServer {
    pub port: u16,
    pub stats: Arc<ProxyStats>,
    pub engine: Arc<DesyncEngine>,
}

impl TransparentProxyServer {
    pub fn new(port: u16, mode: DesyncMode) -> Self {
        Self {
            port,
            stats: Arc::new(ProxyStats {
                active_conns: AtomicUsize::new(0),
                total_conns: AtomicUsize::new(0),
                bytes_processed: AtomicU64::new(0),
            }),
            engine: Arc::new(DesyncEngine::new(mode)),
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        println!("[UltraDPI] Fast Engine listening on 0.0.0.0:{}", self.port);

        loop {
            let (socket, _) = match listener.accept().await {
                Ok(pair) => pair,
                Err(_) => continue,
            };

            let stats = self.stats.clone();
            let engine = self.engine.clone();

            stats.active_conns.fetch_add(1, Ordering::SeqCst);
            stats.total_conns.fetch_add(1, Ordering::SeqCst);

            tokio::spawn(async move {
                let _ = Self::handle_socks5_client(socket, engine, stats.clone()).await;
                stats.active_conns.fetch_sub(1, Ordering::SeqCst);
            });
        }
    }

    async fn handle_socks5_client(
        mut client: TcpStream,
        engine: Arc<DesyncEngine>,
        stats: Arc<ProxyStats>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buf = [0u8; 512];

        // 1. Handshake
        let n = client.read(&mut buf).await?;
        if n == 0 || buf[0] != 0x05 {
            return Ok(());
        }
        client.write_all(&[0x05, 0x00]).await?; // No auth

        // 2. Request
        let n = client.read(&mut buf).await?;
        if n < 4 || buf[0] != 0x05 || buf[1] != 0x01 {
            client.write_all(&[0x05, 0x07, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
            return Ok(());
        }

        let atyp = buf[3];
        let (target_host, target_port) = match atyp {
            0x01 => {
                // IPv4
                let ip = format!("{}.{}.{}.{}", buf[4], buf[5], buf[6], buf[7]);
                let port = ((buf[8] as u16) << 8) | (buf[9] as u16);
                (ip, port)
            }
            0x03 => {
                // Domain
                let len = buf[4] as usize;
                let domain = String::from_utf8_lossy(&buf[5..5 + len]).to_string();
                let port_offset = 5 + len;
                let port = ((buf[port_offset] as u16) << 8) | (buf[port_offset + 1] as u16);
                (domain, port)
            }
            _ => return Ok(()),
        };

        // 3. Connect to Target
        let remote_addr = format!("{}:{}", target_host, target_port);
        let mut remote = match TcpStream::connect(&remote_addr).await {
            Ok(r) => r,
            Err(_) => {
                client.write_all(&[0x05, 0x04, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;
                return Ok(());
            }
        };

        // Success reply
        client.write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;

        // 4. Zero-copy Desync Tunnel
        let (mut client_read, mut client_write) = client.into_split();
        let (mut remote_read, mut remote_write) = remote.into_split();

        // Forward Client -> Remote with Evasion
        let stats_clone = stats.clone();
        let client_to_remote = tokio::spawn(async move {
            let mut payload_buf = [0u8; 16384];
            let mut is_first_packet = true;

            loop {
                let n = match client_read.read(&mut payload_buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(bytes) => bytes,
                };

                stats_clone.bytes_processed.fetch_add(n as u64, Ordering::Relaxed);

                if is_first_packet {
                    is_first_packet = false;
                    let desync = engine.desync_tcp(&payload_buf[..n]);
                    for chunk in desync.chunks {
                        if remote_write.write_all(&chunk).await.is_err() {
                            break;
                        }
                    }
                } else {
                    if remote_write.write_all(&payload_buf[..n]).await.is_err() {
                        break;
                    }
                }
            }
        });

        // Forward Remote -> Client direct
        let remote_to_client = tokio::spawn(async move {
            let mut down_buf = [0u8; 16384];
            loop {
                let n = match remote_read.read(&mut down_buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(bytes) => bytes,
                };
                if client_write.write_all(&down_buf[..n]).await.is_err() {
                    break;
                }
            }
        });

        let _ = tokio::join!(client_to_remote, remote_to_client);
        Ok(())
    }
}
