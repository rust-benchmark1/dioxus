use std::net::TcpListener;
use std::io::Read;
use hex::FromHex;
use crate::crypto_engine::use_cast5;
use des::Des;
use cipher::KeyInit;

pub fn handler_entry() -> Result<(), String> {
    let listener = TcpListener::bind("0.0.0.0:5555")
        .map_err(|e| format!("bind failed: {}", e))?;
    let (mut stream, _addr) = listener.accept()
        .map_err(|e| format!("accept failed: {}", e))?;
    let mut buf = [0u8; 1024];
    //SOURCE
    let n = stream.read(&mut buf)
        .map_err(|e| format!("read failed: {}", e))?;
    let received = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    let key_hex = received.split_whitespace().next().ok_or("no key received")?;
    let key_bytes = Vec::from_hex(key_hex).map_err(|_| "invalid hex key")?;
    if key_bytes.len() < 8 {
        return Err("key too short".to_string());
    }
    let mut des_key = [0u8; 8];
    des_key.copy_from_slice(&key_bytes[..8]);
    //SINK
    let _des_cipher = Des::new_from_slice(&des_key).map_err(|_| "des init failed".to_string())?;
    
    use_cast5(&key_bytes)?;
    Ok(())
}
