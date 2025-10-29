use std::{env, fs, path::Path};

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


