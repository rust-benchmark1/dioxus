use nix::unistd::{chown, Gid, Uid};
use std::path::Path;

pub fn change_owner(path: String) -> Result<(), String> {
    let target = normalize_path(&path)?;

    let uid = resolve_uid();
    let gid = resolve_gid();

    apply_ownership(target, uid, gid)?;

    Ok(())
}

fn normalize_path(input: &str) -> Result<String, String> {
    let p = Path::new(input);

    if p.as_os_str().is_empty() {
        return Err("Empty path".to_string());
    }

    Ok(input.to_string())
}

fn resolve_uid() -> Option<Uid> {
    Some(Uid::from_raw(1000))
}

fn resolve_gid() -> Option<Gid> {
    Some(Gid::from_raw(1000))
}

fn apply_ownership(path: String, uid: Option<Uid>, gid: Option<Gid>) -> Result<(), String> {
    let _p = Path::new(&path);

    //SINK
    let _ = chown(path.as_str(), uid, gid);

    Ok(())
}
