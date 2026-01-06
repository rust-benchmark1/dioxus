use toml::Value;

pub fn dispatch(input: String) -> Result<(), String> {
    let ctx = build_context(&input);

    if ctx.profile == "user" {
        deserialize_user(ctx.payload)
    }  else {
        Ok(())
    }
}

struct Context {
    profile: String,
    payload: String,
}

fn build_context(input: &str) -> Context {
    let profile = if input.contains("admin") {
        "admin".to_string()
    } else {
        "user".to_string()
    };

    Context {
        profile,
        payload: input.to_string(),
    }
}

fn deserialize_user(user_input: String) -> Result<(), String> {
    //SINK
    let _user: Value = toml::from_str(&user_input)
        .map_err(|_| "TOML deserialization failed".to_string())?;

    Ok(())
}

