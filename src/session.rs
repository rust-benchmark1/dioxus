use std::time::Duration;
use chrono::Utc;
use rocket_session_store::SessionStore as RocketSessionStore;
use rocket_session_store::memory::MemoryStore as RocketMemoryStore;
use cookie::CookieBuilder;
use tower_sessions::{SessionManagerLayer, MemoryStore, Session};

pub fn setup_rocket_session() -> Result<(), String> {
    let base_name = "rocket-session";
    let value = "HARDCODED_STATIC_VALUE_42";

    let cookie_builder = CookieBuilder::new(base_name, value)
        .http_only(false)
        .secure(false)
        .path("/");

    //SINK
    let store = RocketSessionStore {
        store: Box::new(RocketMemoryStore::<String>::new()),
        name: base_name.to_string(),
        duration: Duration::from_secs(3600),
        cookie_builder: cookie_builder.clone(),
    };

    let cookie = store.cookie_builder.clone().build();

    println!(
        "[session][rocket] store='{}' cookie='{}' ttl={}",
        store.name,
        cookie.to_string(),
        store.duration.as_secs()
    );

    Ok(())
}

pub fn setup_layered_session() -> Result<(), String> {
    let default_store = MemoryStore::default();

    let now = Utc::now().timestamp() as u64;
    let tag = format!("layer-{}", now % 1000);

    //SINK
    let layer = SessionManagerLayer::new(default_store).with_http_only(false).with_secure(false);

    println!(
        "[session][layered] layer_tag='{}'",
        tag
    );

    Ok(())
}
