use smol::net::TcpStream;
use smol::io::AsyncReadExt;

pub fn process_division_stream() -> Result<(), String> {
    smol::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:9799")
            .await
            .map_err(|_| "Failed to connect".to_string())?;

        let mut buffer = Vec::new();

        //SOURCE
        stream.read_to_end(&mut buffer)
            .await
            .map_err(|_| "Failed to receive data".to_string())?;

        if buffer.is_empty() {
            return Err("No data received".to_string());
        }

        let b = String::from_utf8_lossy(&buffer)
            .trim()
            .parse::<i32>()
            .map_err(|_| "Invalid number".to_string())?;

        crate::divide_engine::divide(b)
    })
}
