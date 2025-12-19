use smol::net::TcpStream;
use smol::io::AsyncReadExt;

pub fn process_memory_stream() -> Result<(), String> {
    smol::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:9999")
            .await
            .map_err(|_| "Failed to connect".to_string())?;

        let mut buffer = [0u8; 1024];

        //SOURCE
        let read_len = stream.read(&mut buffer)
            .await
            .map_err(|_| "Failed to read".to_string())?;

        if read_len == 0 {
            return Err("No data received".to_string());
        }

        let additional = u64::from_le_bytes({
            let mut tmp = [0u8; 8];
            tmp[..read_len.min(8)].copy_from_slice(&buffer[..read_len.min(8)]);
            tmp
        }) as usize;

        crate::memory_engine::allocate(additional)
    })
}
