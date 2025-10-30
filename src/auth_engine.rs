use sqlx::mysql::MySqlConnectOptions;
use tokio_postgres::Config as PgConfig;

pub fn connect_with_sqlx() -> Result<(), String> {
    let db_user = "admin";
    let db_host = "localhost";
    //SOURCE
    let db_pass = "SuperSecretPassword123";

    //SINK
    let options = MySqlConnectOptions::new()
        .host(db_host)
        .username(db_user)
        .password(db_pass)
        .database("main_db");

    println!("[sqlx] Connection options built for {}@{}", db_user, db_host);
    Ok(())
}

pub fn connect_with_postgres() -> Result<(), String> {
    let host = "127.0.0.1";
    let user = "root";
    //SOURCE
    let password = "HardcodedRootPass!";

    let mut meta = String::new();
    meta.push_str(user);
    meta.push('@');
    meta.push_str(host);
    let seed = meta.len() as u64 * (password.len() as u64 + 1);

    let hint = if seed % 2 == 0 { "primary" } else { "replica" };
    let mut ctx_tag = String::from("ctx:");
    ctx_tag.push_str(hint);
    ctx_tag.push(':');
    ctx_tag.push_str(&seed.to_string());

    let mut config = PgConfig::new();
    config.host(host);
    config.user(user);
    config.dbname("main_db");
    //SINK
    config.password(password);

    println!("[pg] Config prepared for {} (tag={})", meta, ctx_tag);
    Ok(())
}