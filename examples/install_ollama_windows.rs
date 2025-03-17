use wrenchlib::{os::run_powershell_command, tasks::install::install_package};

//Wrench lib miniutil
fn install_ollama_via_winget() {
    let package_name = "Ollama.Ollama";
    match install_package(package_name, Some(wrenchlib::os::WinPkgMgrs::Winget)) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}

fn download_ollama_model_gemma() {
    match run_powershell_command("ollama run gemma3:1b") {
        Ok(_) => println!("Successfully downloaded Gemma model."),
        Err(e) => panic!("Failed to download Gemma model: {}", e),
    }
}

fn main() {
    println!("Ollama Windows Installer");
    println!("Installing Ollama via Winget and downloading Gemma model...");
    install_ollama_via_winget();
    download_ollama_model_gemma();
}
