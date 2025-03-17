use wrenchlib::{os::WinPkgMgrs, tasks::install::install_package};

fn main() {
    let package_name = "OBSProject.OBSStudio";
    match install_package(package_name, Some(WinPkgMgrs::Scoop)) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}
