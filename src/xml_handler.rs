use std::net::UdpSocket;

pub fn process_xml_stream() -> Result<String, String> {
    let socket = UdpSocket::bind("0.0.0.0:12121").map_err(|e| e.to_string())?;
    let mut buf = [0u8; 4096];
    //SOURCE
    let (amt, _src) = socket.recv_from(&mut buf).map_err(|e| e.to_string())?;
    if amt == 0 {
        return Err("No data received".to_string());
    }

    let payload = String::from_utf8_lossy(&buf[..amt]).trim().to_string();
    crate::xml_engine::handle_xml_operations(payload)
}
