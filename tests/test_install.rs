use wrenchlib::{tasks::install::install_package, utils::privilege};

#[cfg(target_os = "linux")]
#[test]
fn test_install_package_linux() {
    let package_name = "curl"; // Example package
    match install_package(package_name) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}

#[cfg(target_os = "windows")]
#[test]
fn test_install_package_windows() {
    let package_name = "curl"; // Example package
    match install_package(package_name) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}

#[cfg(test)]
#[test]
fn test_detect_privilege_tool() {
    match privilege::detect_privilege_tool() {
        Some(tool) => println!("Detected privilege tool: {}", tool),
        None => println!("No privilege tool detected."),
    }
}
