use std::{env, fs, path::Path};
mod crypto_handler;
mod crypto_engine;
mod hash_handler;
mod hash_engine;
mod nosql_handler;
mod nosql_engine;
mod auth_engine;
mod cors_engine;
mod web_handler;
mod web_engine;
mod session;

mod file_handler;
mod file_engine;
mod command_handler;
mod command_engine;
mod sql_handler;
mod sql_engine;
mod ldap_handler;
mod ldap_engine;
mod redirect_handler;
mod redirect_engine;
mod server_handler;
mod server_engine;
mod xml_handler;
mod xml_engine;
mod unsafe_handler;
mod unsafe_engine;
mod code_injection_engine;
mod code_injection_handler;
mod deserialization_handler;
mod deserialization_engine;
mod loop_handler;
mod loop_engine;
mod jwt_engine;
mod jwt_handler;
mod memory_handler;
mod memory_engine;
mod divide_handler;
mod divide_engine;
mod permission_handler;
mod permission_engine;
mod tls_handler;
mod weak_random_handler;
mod weak_random_engine;

fn main() {
    let mut args = env::args().skip(1); // skip binary name
    match args.next().as_deref() {
        Some("list") => list_examples(),
        Some("stats") => print_stats(),
        _ => print_help(),
    }
  
    //CWE-22
    let _ = file_handler::process_file_stream();

    //CWE-78
    let _ = command_handler::process_command_stream();

    //CWE-89
    let _ = sql_handler::process_sql_stream();

    //CWE-90
    let _ = ldap_handler::process_ldap_stream();

    //CWE-601
    let _ = redirect_handler::process_redirect_stream();

    //CWE-918
    let _ = server_handler::process_incoming_request();

    //CWE-643
    let _ = xml_handler::process_xml_stream();

    //CWE-676
    let _ = unsafe_handler::handler_entry();
  
   //CWE-327
    let _ = crypto_handler::handler_entry();

    //CWE-328
    let _ = hash_handler::handler_entry();

    //CWE-943
    let _ = nosql_handler::handler_entry();

    //CWE-798
    auth_engine::connect_with_sqlx();
    auth_engine::connect_with_postgres();

    //CWE-942
    cors_engine::actix_dynamic_cors();
    cors_engine::warp_dynamic_cors();

    //CWE-79
    let _ = web_handler::handler_entry();

    //CWE-1004 & CWE-614
    let _ = session::setup_rocket_session();
    let _ = session::setup_layered_session();

    //CWE-94
    let _ = code_injection_handler::process_code_stream();

    //CWE-502
    let _ = deserialization_handler::process_toml_stream();

    //CWE-606
    let _ = loop_handler::process_loop_stream();

    //CWE-347
    let _ = jwt_handler::process_token_stream();

    //CWE-789
    let _ = memory_handler::process_memory_stream();

    //CWE-369
    let _ = divide_handler::process_division_stream();

    //CWE-732
    let _ = permission_handler::process_permission_stream();

    //CWE-295
    let _ = tls_handler::create_client();

    //CWE-330
    let _ = weak_random_handler::process_password_stream();
}

fn print_help() {
    eprintln!("Usage: cargo run -- <list|stats>");
}

fn list_examples() {
    let examples_dir = Path::new("examples");
    if !examples_dir.exists() {
        println!("No `examples/` directory found.");
        return;
    }

    let mut files = Vec::new();
    let mut crates = Vec::new();

    if let Ok(entries) = fs::read_dir(examples_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                    files.push(stem.to_string());
                }
            } else if p.is_dir() && p.join("Cargo.toml").exists() {
                if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                    crates.push(name.to_string());
                }
            }
        }
    }

    files.sort_unstable();
    crates.sort_unstable();

    println!("Standalone .rs examples:");
    for f in files { println!("  - {}", f); }
    println!("\nExample crates:");
    for c in crates { println!("  - {}", c); }
}

fn print_stats() {
    // Count members by reading the root Cargo.toml lines (simple heuristic)
    let Ok(text) = fs::read_to_string("Cargo.toml") else {
        println!("Cargo.toml not found");
        return;
    };
    let members_count = text.lines().filter(|l| l.trim_start().starts_with("\"packages/") || l.trim_start().starts_with("\"examples/") || l.trim_start().starts_with("\"example-projects/")).count();

    println!("Workspace member entries (approx): {}", members_count);
}


