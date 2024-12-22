/// This module contains platform-specific logic for Linux systems.

///
/// It includes functionality for detecting available package managers like `apt`, `pacman`, and `dnf`,
/// as well as installing packages using the appropriate manager.
use std::process::Command;

/// Detects the available package manager on Linux.
/// This function checks for common package managers like `apt`, `pacman`, and others.
///
/// # Returns
///
/// Returns an `Option` with a tuple containing the package manager name and installation syntax,
/// or `None` if no supported package manager is found.
pub fn detect_package_manager() -> Option<(&'static str, &'static [&'static str])> {
    let managers = vec![
        ("apt", &["install", "-y"] as &[_]),
        ("pacman", &["-S", "--noconfirm"]),
        ("dnf", &["install", "-y"]),
        ("yum", &["install", "-y"]),
        ("zypper", &["install", "--non-interactive"]),
    ];

    for (manager, syntax) in managers {
        if Command::new(manager).output().is_ok() {
            return Some((manager, syntax));
        }
    }
    None
}

/// Installs a package on Linux using the detected package manager.
///
/// # Arguments
/// * `package_name` - The name of the package to install.
///
/// # Returns
/// * `Ok(())` if the package is successfully installed.
/// * `Err(String)` if an error occurs during installation, containing the error message.
pub fn install_package(package_name: &str) -> Result<(), String> {
    if let Some((manager, syntax)) = detect_package_manager() {
        let args: Vec<&str> = syntax.iter().chain(&[package_name]).map(|s| *s).collect();
        super::super::utils::privilege::run_as_privileged(manager, &args)
    } else {
        Err("No supported package manager found.".to_string())
    }
}
