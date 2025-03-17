use wrenchlib::tasks::install::install_package;

fn main() {
    let package_name = "OBSProject.OBSStudio";
    match install_package(package_name, wrenchlib::os::WinPkgMgrs::Winget) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}
