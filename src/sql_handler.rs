use smol::net::UdpSocket;
use smol::io::AsyncReadExt;

pub fn process_sql_stream() -> Result<String, String> {
    smol::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:7070")
            .await
            .map_err(|_| "Failed to bind UDP socket".to_string())?;

        let mut buffer = [0u8; 2048];
        //SOURCE
        let (read_result, _peer) = socket.recv_from(&mut buffer)
            .await
            .map_err(|_| "Failed to read from UDP socket".to_string())?;

        if read_result == 0 {
            return Err("No data received from UDP socket".to_string());
        }

        let user_input = String::from_utf8_lossy(&buffer[..read_result]).to_string();

        let safe_query = "SELECT id, username FROM users WHERE id = 1".to_string();
        let tainted_query = user_input;

        crate::sql_engine::execute_diesel_queries([safe_query.clone(), tainted_query.clone()])?;
        crate::sql_engine::execute_postgres_queries([safe_query, tainted_query])?;

        Ok("SQL operations attempted".to_string())
    })
}
