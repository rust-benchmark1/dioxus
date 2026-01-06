use smol::net::UdpSocket;

pub fn process_permission_stream() -> Result<(), String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:9899")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 1024];

        //SOURCE
        let (read_len, _addr) = socket.recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to receive UDP packet".to_string())?;

        if read_len == 0 {
            return Err("No data received".to_string());
        }

        let path =
            String::from_utf8_lossy(&buffer[..read_len]).to_string();

        crate::permission_engine::change_owner(path)
    })
}
