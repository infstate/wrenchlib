#[cfg(target_os = "windows")]
#[test]
fn test_doom_emacs_scoop_add_bucket() {
    use wrenchlib::os::windows::scoop_add_bucket;
    let bucket_name = "extras";
    match scoop_add_bucket(wrenchlib::os::ScoopBucket::Extras) {
        Ok(_) => println!("Successfully added bucket {}.", bucket_name),
        Err(e) => panic!("Failed to add bucket {}: {}", bucket_name, e),
    }
}
#[cfg(target_os = "windows")]
#[test]
fn test_doom_emacs_install_scoop() {
    use wrenchlib::tasks::install::install_package;

    let package_name = "git emacs ripgrep fd llvm";
    match install_package(package_name, wrenchlib::os::WinPkgMgrs::Scoop) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}

// #[cfg(target_os = "windows")]
// #[test]
// fn test_doom_emacs_raw_powershell() {
//     use wrenchlib::os::windows::fetch_powershell_version;
//     match fetch_powershell_version() {
//         Ok(_) => println!("Successfully fetched PowerShell version."),
//         Err(e) => panic!("Failed to fetch PowerShell version: {}", e),
//     }
// }

#[cfg(target_os = "windows")]
#[test]
fn test_doom_emacs_clone_repo() {
    use wrenchlib::os::windows::git_clone_repo;
    match git_clone_repo("https://github.com/doomemacs/doomemacs", "~/.config/emacs") {
        Ok(_) => println!("Successfully cloned Doom Emacs."),
        Err(e) => panic!("Failed to clone Doom Emacs: {}", e),
    }
}

#[cfg(target_os = "windows")]
#[test]
fn test_doom_emacs_run() {
    use wrenchlib::os::windows::run_powershell_command;
    match run_powershell_command("~/.config/emacs/bin/doom install") {
        Ok(_) => println!("Success!"),
        Err(e) => panic!("Failed to run powershell"),
    }
}
