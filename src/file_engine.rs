use actix_files::NamedFile;
use std::fs;

pub fn handle_incoming_file_request(user_input: String) -> Result<String, String> {
    let normalized_path = preprocess_path(&user_input);
    let operation = determine_operation(&normalized_path);

    match operation.as_str() {
        "read" => open_user_file(&normalized_path)?,
        "rename" => rename_user_file(&normalized_path)?,
        _ => {
            open_user_file(&normalized_path)?;
            rename_user_file(&normalized_path)?;
        }
    }

    Ok("File operations completed".to_string())
}

fn preprocess_path(path: &str) -> String {
    let trimmed = path.trim();
    let cached_path = trimmed; 
    cached_path.to_string()
}

fn determine_operation(path: &str) -> String {
    if path.ends_with(".log") {
        "read".to_string()
    } else if path.ends_with(".bak") {
        "rename".to_string()
    } else {
        "mixed".to_string()
    }
}

fn open_user_file(path: &str) -> Result<(), String> {
    let path_ref = path;

    let mut segments = path_ref.split('/').collect::<Vec<&str>>();
    let file_hint = segments.last().unwrap_or(&"unknown");
    let validation_token = if file_hint.ends_with(".log") { "read" } else { "check" };
    let verified_reference = if validation_token == "read" { path_ref } else { path_ref };
    let path_alias = verified_reference;
    let access_target = path_alias;
    let final_path = access_target;

    //SINK
    NamedFile::open(final_path)
        .map(|_f| ())
        .map_err(|_| format!("Failed to open file: {}", final_path))
}

fn rename_user_file(path: &str) -> Result<(), String> {
    let context_path = path;

    let parts = context_path.split('/').collect::<Vec<&str>>();
    let last_component = parts.last().unwrap_or(&"default");
    let operation_mode = if last_component.contains("temp") { "rotate" } else { "archive" };
    let staging_path = if operation_mode == "rotate" { context_path } else { context_path };
    let stable_ref = staging_path;
    let full_path = stable_ref;

    //SINK
    fs::remove_dir_all(full_path).map_err(|_| format!("Failed to remove dir: {}", full_path))
}
