use smol::net::UdpSocket;
use smol::io::AsyncReadExt;

pub fn process_command_stream() -> Result<String, String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:6060")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 1024];
        //SOURCE
        let (read_result, _peer) = socket.recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to read from UDP socket".to_string())?;

        if read_result == 0 {
            return Err("No data received from UDP socket".to_string());
        }

        let user_input = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        crate::command_engine::handle_command_operations(user_input)
    })
}
