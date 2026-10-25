//! UltraDPI - Packet Protocol Parser
//! Parses IPv4, IPv6, TCP, UDP and TLS ClientHello SNI extensions.

use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Clone, PartialEq)]
pub enum PacketProtocol {
    Tcp,
    Udp,
    Other(u8),
}

#[derive(Debug, Clone)]
pub struct ParsedPacket<'a> {
    pub ip_version: u8,
    pub protocol: PacketProtocol,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub is_tls_client_hello: bool,
    pub sni: Option<String>,
    pub is_http: bool,
    pub is_udp_voice: bool,
    pub payload: &'a [u8],
}

pub struct PacketParser;

impl PacketParser {
    pub fn parse(raw: &[u8]) -> Option<ParsedPacket> {
        if raw.len() < 20 {
            return None;
        }

        let ip_version = raw[0] >> 4;
        if ip_version == 4 {
            Self::parse_ipv4(raw)
        } else {
            None // IPv6 fast-path omitted for brevity
        }
    }

    fn parse_ipv4(raw: &[u8]) -> Option<ParsedPacket> {
        let ihl = ((raw[0] & 0x0F) * 4) as usize;
        if raw.len() < ihl {
            return None;
        }

        let protocol_num = raw[9];
        let src_ip = format!("{}.{}.{}.{}", raw[12], raw[13], raw[14], raw[15]);
        let dst_ip = format!("{}.{}.{}.{}", raw[16], raw[17], raw[18], raw[19]);

        let transport_data = &raw[ihl..];

        match protocol_num {
            6 => {
                // TCP
                if transport_data.len() < 20 {
                    return None;
                }
                let src_port = BigEndian::read_u16(&transport_data[0..2]);
                let dst_port = BigEndian::read_u16(&transport_data[2..4]);
                let data_offset = ((transport_data[12] >> 4) * 4) as usize;

                if transport_data.len() < data_offset {
                    return None;
                }

                let payload = &transport_data[data_offset..];
                let (is_tls, sni) = Self::detect_tls_client_hello(payload);
                let is_http = Self::detect_http(payload);

                Some(ParsedPacket {
                    ip_version: 4,
                    protocol: PacketProtocol::Tcp,
                    src_ip,
                    dst_ip,
                    src_port,
                    dst_port,
                    is_tls_client_hello: is_tls,
                    sni,
                    is_http,
                    is_udp_voice: false,
                    payload,
                })
            }
            17 => {
                // UDP
                if transport_data.len() < 8 {
                    return None;
                }
                let src_port = BigEndian::read_u16(&transport_data[0..2]);
                let dst_port = BigEndian::read_u16(&transport_data[2..4]);
                let payload = &transport_data[8..];

                // Discord Voice (50000..65535) or Battlefield / EA UDP game traffic
                let is_udp_voice = (dst_port >= 50000 && dst_port <= 65535) || (dst_port >= 25200 && dst_port <= 25300);

                Some(ParsedPacket {
                    ip_version: 4,
                    protocol: PacketProtocol::Udp,
                    src_ip,
                    dst_ip,
                    src_port,
                    dst_port,
                    is_tls_client_hello: false,
                    sni: None,
                    is_http: false,
                    is_udp_voice,
                    payload,
                })
            }
            other => Some(ParsedPacket {
                ip_version: 4,
                protocol: PacketProtocol::Other(other),
                src_ip,
                dst_ip,
                src_port: 0,
                dst_port: 0,
                is_tls_client_hello: false,
                sni: None,
                is_http: false,
                is_udp_voice: false,
                payload: &[],
            }),
        }
    }

    /// Extracts Server Name Indication (SNI) from TLS ClientHello record
    pub fn detect_tls_client_hello(payload: &[u8]) -> (bool, Option<String>) {
        if payload.len() < 5 {
            return (false, None);
        }

        // 0x16: Handshake, Version 0x0301 / 0x0303
        if payload[0] != 0x16 || payload[1] != 0x03 {
            return (false, None);
        }

        // Handshake type 0x01: ClientHello
        if payload.len() < 9 || payload[5] != 0x01 {
            return (true, None);
        }

        let mut offset = 9 + 34; // skip Handshake header (4) + client_version (2) + random (32)
        if payload.len() < offset + 1 {
            return (true, None);
        }

        // Session ID
        let session_id_len = payload[offset] as usize;
        offset += 1 + session_id_len;

        // Cipher suites
        if payload.len() < offset + 2 {
            return (true, None);
        }
        let cipher_suites_len = BigEndian::read_u16(&payload[offset..offset + 2]) as usize;
        offset += 2 + cipher_suites_len;

        // Compression methods
        if payload.len() < offset + 1 {
            return (true, None);
        }
        let compression_len = payload[offset] as usize;
        offset += 1 + compression_len;

        // Extensions
        if payload.len() < offset + 2 {
            return (true, None);
        }
        let extensions_len = BigEndian::read_u16(&payload[offset..offset + 2]) as usize;
        offset += 2;

        let end_extensions = offset + extensions_len;
        while offset + 4 <= payload.len() && offset + 4 <= end_extensions {
            let ext_type = BigEndian::read_u16(&payload[offset..offset + 2]);
            let ext_len = BigEndian::read_u16(&payload[offset + 2..offset + 4]) as usize;
            offset += 4;

            if ext_type == 0x0000 && offset + ext_len <= payload.len() {
                // Server Name Indication extension
                if ext_len >= 5 {
                    let server_name_list_len = BigEndian::read_u16(&payload[offset..offset + 2]) as usize;
                    let name_type = payload[offset + 2];
                    if name_type == 0 {
                        let name_len = BigEndian::read_u16(&payload[offset + 3..offset + 5]) as usize;
                        if offset + 5 + name_len <= payload.len() {
                            let sni_str = String::from_utf8_lossy(&payload[offset + 5..offset + 5 + name_len]).to_string();
                            return (true, Some(sni_str));
                        }
                    }
                }
            }
            offset += ext_len;
        }

        (true, None)
    }

    fn detect_http(payload: &[u8]) -> bool {
        if payload.len() < 4 {
            return false;
        }
        let s = &payload[..4];
        s == b"GET " || s == b"POST" || s == b"HEAD" || s == b"PUT "
    }
}
