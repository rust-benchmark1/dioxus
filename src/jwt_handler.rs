use smol::net::UdpSocket;

pub fn process_token_stream() -> Result<(), String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:9898")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 4096];

        //SOURCE
        let (read_len, _addr) = socket.recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to receive UDP message".to_string())?;

        if read_len == 0 {
            return Err("No data received".to_string());
        }

        let token = String::from_utf8_lossy(&buffer[..read_len]).to_string();

        crate::jwt_engine::process_token(token)
    })
}
