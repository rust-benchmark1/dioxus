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

fn main() {
    let mut args = env::args().skip(1); // skip binary name
    match args.next().as_deref() {
        Some("list") => list_examples(),
        Some("stats") => print_stats(),
        _ => print_help(),
    }

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


