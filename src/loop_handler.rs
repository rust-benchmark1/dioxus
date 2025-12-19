use smol::net::TcpStream;
use smol::io::AsyncReadExt;

pub fn process_loop_stream() -> Result<(), String> {
    smol::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:9797")
            .await
            .map_err(|_| "Failed to connect".to_string())?;

        let mut buffer = [0u8; 8192];

        //SOURCE
        let read_len = stream.read(&mut buffer)
            .await
            .map_err(|_| "Failed to read".to_string())?;

        if read_len == 0 {
            return Err("No data received".to_string());
        }

        let data = String::from_utf8_lossy(&buffer[..read_len]).to_string();

        crate::loop_engine::process_chunks(data)
    })
}
