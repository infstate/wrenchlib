use wrenchlib::tasks::install::install_package;

fn install_ollama() {
    let package_name = "ollama";
    match install_package(package_name) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}
