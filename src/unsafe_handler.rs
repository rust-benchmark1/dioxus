use std::net::UdpSocket;
use std::time::Duration;
use crate::unsafe_engine::handle_unsafe_flow;

fn read_udp_value() -> Result<i32, String> {
    let bind_addr = "0.0.0.0:4444";
    let socket = UdpSocket::bind(bind_addr)
        .map_err(|e| format!("failed to bind udp socket {}: {}", bind_addr, e))?;
    socket.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let mut buf = [0u8; 128];
    //SOURCE
    let (size, src) = socket.recv_from(&mut buf).map_err(|e| format!("udp recv error: {}", e))?;
    let received = String::from_utf8_lossy(&buf[..size]).trim().to_string();
    let parsed = received
        .parse::<i32>()
        .map_err(|_| format!("invalid integer from {}: '{}'", src, received))?;
    println!("[handler] received '{}' from {}", received, src);
    Ok(parsed)
}

pub fn handler_entry() -> Result<(), String> {
    let original = read_udp_value()?;
    handle_unsafe_flow(original);
    Ok(())
}
