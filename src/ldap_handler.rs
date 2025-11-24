use smol::net::TcpStream;
use smol::io::AsyncReadExt;

pub fn process_ldap_stream() -> Result<String, String> {
    smol::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:9191")
            .await
            .map_err(|_| "Failed to connect to TCP source".to_string())?;

        let mut buffer = [0u8; 2048];
        //SOURCE
        let read_len = stream.read(&mut buffer)
            .await
            .map_err(|_| "Failed to read from TCP stream".to_string())?;

        if read_len == 0 {
            return Err("No data received from TCP stream".to_string());
        }

        let data = String::from_utf8_lossy(&buffer[..read_len]).to_string();
        crate::ldap_engine::handle_ldap_operations(data)
    })
}
