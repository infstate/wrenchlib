#[cfg(target_os = "linux")]
use crate::os::linux::install_package as os_install_package;

#[cfg(target_os = "linux")]
pub fn install_package(package_name: &str, manager: Option<String>) -> Result<(), String> {
    let package_manager = match manager {
        Some(mgr) => mgr,
        None => match os::linux::detect_package_manager() {
            Ok((mgr, _)) => mgr.to_string(),
            Err(e) => return Err(format!("Failed to detect package manager: {}", e)),
        },
    };

    os_install_package(package_name, package_manager)
}

// use crate::tasks::collect::collect_system_info;

#[cfg(target_os = "windows")]
use crate::os::windows::install_package as os_install_package;
use crate::os::windows::WinPkgMgrs;

/// Installs a package on the system using the appropriate package manager for the platform (Linux or Windows).
///
/// This function will detect the platform and use the correct package manager to install the specified package.
///
/// # Arguments
/// * `package_name` - The name of the package to install.
///
/// # Returns
///
/// * `Ok(())` if the package was successfully installed.
/// * `Err(String)` if an error occurs during installation, containing the error message.
///
#[cfg(target_os = "windows")]
pub fn install_package(package_name: &str, manager: Option<WinPkgMgrs>) -> Result<(), String> {
    os_install_package(package_name, manager.unwrap_or(WinPkgMgrs::Winget))
}

// pub fn find_package_name(package_name: &str) -> Result<String, String> {
//     Ok(package_name.to_string())
// }
