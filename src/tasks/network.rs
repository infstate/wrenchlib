#[cfg(target_os = "windows")]
use crate::os::run_powershell_command;

#[cfg(target_os = "windows")]
pub fn download_file_powershell(url: &str, destination: &str) -> Result<(), String> {
    let command = format!(
        "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
        url, destination
    );
    match run_powershell_command(&command) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to download file: {}", e)),
    }
}

pub fn download_file_curl(url: &str, destination: &str) -> Result<(), String> {
    let command = format!("curl -o '{}' '{}'", destination, url);
    match run_powershell_command(&command) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to download file: {}", e)),
    }
}

pub fn download_file_wget(url: &str, destination: &str) -> Result<(), String> {
    let command = format!("wget -O '{}' '{}'", destination, url);
    match run_powershell_command(&command) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to download file: {}", e)),
    }
}
