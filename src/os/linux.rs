/// This module contains platform-specific logic for Linux systems.

///
/// It includes functionality for detecting available package managers like `apt`, `pacman`, and `dnf`,
/// as well as installing packages using the appropriate manager.
use std::process::Command;
//Make a type that works with like in detect_package_manger
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
        ("zypper", &["install", "-y"]),
        ("apt-get", &["install", "-y"]),
        ("apk", &["add"]),
        ("flatpak", &["install", "--non-interactive"]),
        ("snap", &["install"]),
        ("brew", &["install"]),
        ("nix-env", &["-iA"]),
        ("cargo", &["install"]),
        ("pip3", &["install"]),
        ("gem", &["install"]),
        ("conda", &["install"]),
        ("port", &["install"]),
    ];

    for (manager, syntax) in managers {
        // Check if the command can be executed
        if Command::new(manager).output().is_ok() {
            return Some((manager, syntax));
        }
    }
    None
}

pub fn collect_system_info() {
    // Detect the package manager and store it in the SystemInfo struct
    // let package_manager = detect_package_manager();
    // let system_info = SystemInfo {
    //     package_manager: package_manager.map(|pm| format!("{:?}", pm)),
    // };
    // println!("System Info: {:?}", system_info);
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
