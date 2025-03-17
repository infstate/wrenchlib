#[cfg(target_os = "linux")]
use crate::os::linux::install_package as os_install_package;

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
pub fn install_package(package_name: &str, manager: WinPkgMgrs) -> Result<(), String> {
    os_install_package(package_name, manager)
}
