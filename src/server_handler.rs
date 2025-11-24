use std::net::UdpSocket;

pub fn process_incoming_request() -> Result<String, String> {
    let socket = UdpSocket::bind("0.0.0.0:10101").map_err(|e| e.to_string())?;
    let mut buf = [0u8; 2048];
    //SOURCE
    let (amt, _src) = socket.recv_from(&mut buf).map_err(|e| e.to_string())?;
    if amt == 0 {
        return Err("No data received".to_string());
    }

    let input = String::from_utf8_lossy(&buf[..amt]).trim().to_string();
    crate::server_engine::handle_http_requests(input)
}
