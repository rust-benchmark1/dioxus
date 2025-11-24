use std::io::Read;
use std::net::TcpListener;

use crate::web_engine;

pub fn handler_entry() -> Result<(), String> {
    let listener = TcpListener::bind("0.0.0.0:8088")
        .map_err(|e| format!("bind failed: {}", e))?;
    let (mut stream, _addr) = listener.accept()
        .map_err(|e| format!("accept failed: {}", e))?;

    let mut buf = [0u8; 2048];
    //SOURCE
    let n = stream.read(&mut buf).map_err(|e| format!("read failed: {}", e))?;
    let received = String::from_utf8_lossy(&buf[..n]).trim().to_string();

    web_engine::poem_xml(&received)?;
    web_engine::axum_html(&received)?;

    Ok(())
}
