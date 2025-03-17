use wrenchlib::os::run_powershell_command;
use wrenchlib::os::windows::git_clone_repo;
use wrenchlib::os::windows::scoop_add_bucket;
use wrenchlib::os::windows::ScoopBucket;
use wrenchlib::os::WinPkgMgrs;
use wrenchlib::tasks::install::install_package;

fn ensure_git_installed() {
    let package_name = "git";
    match install_package(package_name, Some(WinPkgMgrs::Scoop)) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}
fn add_scoop_bucket() {
    match scoop_add_bucket(ScoopBucket::Extras) {
        Ok(_) => println!("Successfully added bucket."),
        Err(e) => panic!("Failed to add bucket: {}", e),
    }
}

fn install_emacs() {
    let package_name = "git emacs ripgrep fd llvm";
    match install_package(package_name, Some(WinPkgMgrs::Scoop)) {
        Ok(_) => println!("Successfully installed {}.", package_name),
        Err(e) => panic!("Failed to install {}: {}", package_name, e),
    }
}

fn git_clone_doom_emacs() {
    match git_clone_repo("https://github.com/doomemacs/doomemacs", ".config/emacs") {
        Ok(_) => println!("Successfully cloned Doom Emacs."),
        Err(e) => panic!("Failed to clone Doom Emacs: {}", e),
    }
}

fn run_doom_emacs() {
    match run_powershell_command(".config/emacs/bin/doom.ps1 install") {
        Ok(_) => println!("Success!"),
        Err(_e) => panic!("Failed to run powershell"),
    }
}
fn main() {
    ensure_git_installed();
    add_scoop_bucket();
    install_emacs();
    git_clone_doom_emacs();
    run_doom_emacs();
}
