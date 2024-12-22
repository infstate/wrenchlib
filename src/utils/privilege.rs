use std::process::{Command, Stdio};

/// Detects which privilege escalation tool is available on the system.
///
/// This function checks for the presence of either `sudo` or `doas`, which are used to run commands with elevated privileges.
///
/// # Returns
///
/// Returns an `Option` with the name of the available privilege tool (e.g., "sudo" or "doas"), or `None` if neither is available.
pub fn detect_privilege_tool() -> Option<&'static str> {
    let tools = vec!["sudo", "doas"];
    for tool in tools {
        if Command::new(tool).output().is_ok() {
            return Some(tool);
        }
    }
    None
}

/// Runs a command as a privileged user using the detected privilege tool.
///
/// # Arguments
/// * `command` - The command to run (e.g., "apt").
/// * `args` - The arguments to pass to the command (e.g., `["install", "-y", "package_name"]`).
///
/// # Returns
///
/// * `Ok(())` if the command was successfully run.
/// * `Err(String)` if an error occurs while attempting to run the command.
pub fn run_as_privileged(command: &str, args: &[&str]) -> Result<(), String> {
    if let Some(tool) = detect_privilege_tool() {
        let mut cmd = Command::new(tool);
        cmd.arg(command)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let status = cmd.status();
        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err(format!("Command failed with {}.", tool)),
            Err(e) => Err(format!("Error running {}: {}", tool, e)),
        }
    } else {
        Err("Neither 'sudo' nor 'doas' is available. Please install one to proceed.".to_string())
    }
}
