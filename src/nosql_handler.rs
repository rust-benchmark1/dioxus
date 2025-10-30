use std::net::UdpSocket;
use std::time::Duration;
use crate::nosql_engine;

pub fn handler_entry() -> Result<(), String> {
    let bind_addr = "0.0.0.0:7000";
    let socket = UdpSocket::bind(bind_addr).map_err(|e| format!("bind failed: {}", e))?;
    socket.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let mut buf = [0u8; 2048];

    //SOURCE
    let (size, _src) = socket.recv_from(&mut buf).map_err(|e| format!("recv failed: {}", e))?;
    let received = String::from_utf8_lossy(&buf[..size]).trim().to_string();

    let safe0 = "meta:static_value".to_string();
    let tainted = received.clone();
    let safe2 = "static_base64_value".to_string();

    let items = [safe0, tainted, safe2];

    nosql_engine::mongo_write(items.clone())?;
    nosql_engine::dynamo_write(items)?;

    Ok(())
}
