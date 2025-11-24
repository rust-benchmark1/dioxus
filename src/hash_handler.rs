use std::net::UdpSocket;
use std::time::Duration;
use crate::hash_engine::{compute_md5, compute_md2};

pub fn handler_entry() -> Result<(), String> {
    let bind_addr = "0.0.0.0:6148";
    let socket = UdpSocket::bind(bind_addr)
        .map_err(|e| format!("failed to bind udp socket {}: {}", bind_addr, e))?;
    socket.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let mut buf = [0u8; 1024];
    //SOURCE
    let (size, _src) = socket.recv_from(&mut buf)
        .map_err(|e| format!("udp recv error: {}", e))?;
    let received = String::from_utf8_lossy(&buf[..size]).trim().to_string();

    let preview = if received.len() > 64 { &received[..64] } else { &received };
    let meta = format!("len{}_preview{}", received.len(), preview.len());
    let tag = meta.clone();

    let _md5 = compute_md5(&received, &tag)?;
    let _md2 = compute_md2(&received, &meta)?;

    Ok(())
}
