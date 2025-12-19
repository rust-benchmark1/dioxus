use deno_core::{JsRuntime, RuntimeOptions};

pub fn execute_code(code: String) -> Result<(), String> {
    let mut rt = JsRuntime::new(RuntimeOptions::default());

    //SINK
    rt.execute_script("<anon>", code)
        .map_err(|e| format!("Execution failed: {:?}", e))?;

    Ok(())
}
