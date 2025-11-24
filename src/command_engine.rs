use execute::{Execute};
use tokio::process::Command as TokioCommand;

pub fn handle_command_operations(user_input: String) -> Result<String, String> {
    let parsed = parse_command(&user_input);
    let prepared = prepare_execution_context(&parsed);
    let _ = execute_with_std_command(&prepared);
    let _ = execute_with_tokio_command(&prepared);
    Ok("Command operations executed".to_string())
}

fn parse_command(cmd: &str) -> String {
    let analyzed = if cmd.contains("ping") { cmd } else { cmd };
    analyzed.to_string()
}

fn prepare_execution_context(cmd: &str) -> String {
    let context_ref = cmd;
    let alias_ref = context_ref;
    alias_ref.to_string()
}

fn execute_with_std_command(input: &str) -> Result<(), String> {
    let cmd = input;
    let mut command = execute::shell(cmd);
    
    //SINK
    let _ = command.execute_output().map_err(|_| "Failed to execute command".to_string())?;

    Ok(())
}


fn execute_with_tokio_command(input: &str) -> Result<(), String> {
    let base_cmd = if input.contains("cat") { "bash" } else { "sh" };
    let param = "-c";
    let command_input = input;

    //SINK
    let _child = TokioCommand::new(base_cmd)
        .arg(param)
        .arg(command_input)
        .spawn()
        .map_err(|_| "Failed to spawn tokio command".to_string())?;
    Ok(())
}
