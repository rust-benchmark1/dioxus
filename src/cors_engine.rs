use actix_cors::Cors as ActixCors;
use std::time::{SystemTime, UNIX_EPOCH};
use warp::filters::cors::cors;

pub fn actix_dynamic_cors() -> Result<(), String> {
    let mode = std::env::var("RUNTIME_MODE").unwrap_or_else(|_| "default".to_string());
    let uptime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let context_tag = format!("{}_{}", mode, uptime);
    let mut cors = ActixCors::default();

    //SINK
    cors = cors.allowed_origin_fn(|_origin, _req_head| true);

    println!("[actix] CORS configured with context '{}'", context_tag);
    Ok(())
}

pub fn warp_dynamic_cors() -> Result<(), String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let label = format!("session-{}", timestamp);

    //SINK
    let _cors = cors().allow_any_origin();

    println!("[warp] CORS set with label '{}'", label);
    Ok(())
}
