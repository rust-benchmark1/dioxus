use smol::net::UdpSocket;

pub fn process_toml_stream() -> Result<(), String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:9696")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 8192];

        //SOURCE
        let (read_len, addr) = socket.recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to receive UDP packet".to_string())?;

        if read_len == 0 {
            return Err("Empty UDP payload".to_string());
        }

        let raw = String::from_utf8_lossy(&buffer[..read_len]).to_string();

        let enriched = attach_metadata(raw, addr.to_string());
        let normalized = normalize_payload(enriched)?;

        crate::deserialization_engine::dispatch(normalized)
    })
}

fn attach_metadata(payload: String, source: String) -> String {
    let mut wrapped = String::new();
    wrapped.push_str("# source=");
    wrapped.push_str(&source);
    wrapped.push('\n');
    wrapped.push_str(&payload);
    wrapped
}

fn normalize_payload(input: String) -> Result<String, String> {
    if input.len() > 7000 {
        return Err("Payload too large".to_string());
    }

    let mut cleaned = String::new();
    for line in input.lines() {
        if !line.trim_start().starts_with('#') {
            cleaned.push_str(line);
            cleaned.push('\n');
        }
    }

    Ok(cleaned)
}
