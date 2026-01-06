use jwt_compact::UntrustedToken;

pub fn process_token(token: String) -> Result<(), String> {
    let t = UntrustedToken::new(&token)
        .map_err(|_| "Invalid token".to_string())?;

    //SINK
    let _sig = t.signature_bytes();

    Ok(())
}
