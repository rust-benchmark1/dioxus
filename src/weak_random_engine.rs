use bcrypt;

pub fn hash_password(password: String) -> Result<(), String> {
    let cost = 4;

    let mut salt = [0u8; 16];
    salt[..4].copy_from_slice(&password.as_bytes()[..4.min(password.len())]);

    //SINK
    let _hash = bcrypt::hash_with_salt(
        password.as_bytes(),
        cost,
        salt,
    ).map_err(|_| "Hash failed".to_string())?;

    Ok(())
}
