use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use serde_json::Value;

const MAX_IP_LENGTH: usize = 45 as usize;

async fn read_packet(mut stream: TcpStream) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut byte = [0; 1];
    let mut trailing_zeros = false;

    stream.set_read_timeout(Some(Duration::from_millis(200)))
        .expect("Failed to set read timeout");

    loop {
        match stream.read_exact(&mut byte) {
            Ok(()) => {
                if byte[0] == 0 {
                    if trailing_zeros {
                        break;
                    } else {
                        trailing_zeros = true;
                    }
                } else {
                    trailing_zeros = false;
                    buffer.push(byte[0]);
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    buffer
}

fn build_payload(data: &str, port: u16) -> [u8; MAX_IP_LENGTH] {
    let ip_len = data.len() as u8;
    let mut payload = [0u8; MAX_IP_LENGTH];

    payload[0] = (data.len() + 6) as u8;
    payload[1] = 0x00;
    payload[2] = 0x2f;
    payload[3] = ip_len;
    payload[4..4 + data.len()].copy_from_slice(data.as_bytes());
    payload[4 + data.len()] = (port >> 8) as u8;
    payload[4 + data.len() + 1] = port as u8;
    payload[4 + data.len() + 2] = 0x01;
    payload[4 + data.len() + 3] = 0x01;
    payload[4 + data.len() + 4] = 0x00;

    payload
}

pub(crate) async fn get_server_info(ip: &str, port: u16) -> serde_json::Result<serde_json::Value> {
    let mut stream = TcpStream::connect((ip, port))
        .expect("Error via connection to server");
    let ping_payload = build_payload(ip, port);

    stream.write_all(&ping_payload)
        .expect("Failed to send data to server");

    let response = read_packet(stream).await;
    let response_str = std::str::from_utf8(&response[4..])
        .expect("Failed to convert response to UTF-8 string");
    let response_json: Value = serde_json::from_str(response_str)?;

    Ok(response_json)
}