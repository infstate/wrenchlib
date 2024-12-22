use std::process::{Command, Stdio};

/// Checks if the current user has elevated privileges (administrator) on Windows.
///
/// This function uses PowerShell to check if the current user is part of the `Administrators` group.
///
/// # Returns
///
/// Returns `true` if the user is an administrator, `false` otherwise.
pub fn is_elevated() -> bool {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("[Security.Principal.WindowsPrincipal]::Current.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)")
        .output();

    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "True",
        Err(_) => false,
    }
}

pub fn install_package(package_name: &str) -> Result<(), String> {
    if is_elevated() {
        let status = Command::new("winget")
            .arg("install")
            .arg(package_name)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("Package installation failed.".to_string()),
            Err(e) => Err(format!("Error running winget: {}", e)),
        }
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}
