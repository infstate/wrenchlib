use std::process::{Command, Stdio};
#[derive(Debug)]
pub enum WinPkgMgrs {
    Winget,
    Scoop,
    Choco,
}

#[derive(Debug)]
pub enum ScoopBucket {
    Main,
    Extras,
    Versions,
    Nirsoft,
    Sysinternals,
    Php,
    NerdFonts,
    Nonportable,
    Java,
    Games,
}

/// Checks if the current user has elevated privileges (administrator) on Windows.
///
/// This function uses PowerShell to check if the current user is part of the `Administrators` group.
///
/// # Returns
///
/// Returns `true` if the user is an administrator, `false` otherwise.
pub fn is_elevated() -> bool {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] \"Administrator\")")
        .output();

    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "True",
        Err(_) => false,
    }
}

pub fn detect_package_manager_windows() -> Option<WinPkgMgrs> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Command winget, scoop, choco -ErrorAction SilentlyContinue")
        .output();

    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("winget") {
                Some(WinPkgMgrs::Winget)
            } else if output_str.contains("scoop") {
                Some(WinPkgMgrs::Scoop)
            } else if output_str.contains("choco") {
                Some(WinPkgMgrs::Choco)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
// pub fn collect_system_info() {
//     // Detect the package manager and store it in the SystemInfo struct
//     let package_manager = detect_package_manager_windows();
//     let system_info = SystemInfo {
//         package_manager: Some(package_manager.unwrap_or(WinPkgMgrs::Winget)),
//     };
//     println!("System Info: {:?}", system_info);
// }
pub fn fetch_powershell_version() -> Result<(), String> {
    if is_elevated() {
        let status = Command::new("powershell")
            .arg("-Command")
            .arg("$PSVersionTable.PSVersion")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("Powershell version fetch failed.".to_string()),
            Err(e) => Err(format!("Error running powershell: {}", e)),
        }
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}
// pub fn fetch_winget_package(package_name: &str) -> Result<(), String> {
//     if is_elevated() {
//         let status = Command::new("winget")
//             .arg("show")
//             .arg(package_name)
//             .stdout(Stdio::inherit())
//             .stderr(Stdio::inherit())
//             .status();

//         match status {
//             Ok(status) if status.success() => Ok(()),
//             Ok(_) => Err("Package installation failed.".to_string()),
//             Err(e) => Err(format!("Error running winget: {}", e)),
//         }
//     } else {
//         Err("Please rerun this program as Administrator.".to_string())
//     }
// }
pub fn fetch_powershell_execution_policy() -> Result<(), String> {
    if is_elevated() {
        let status = Command::new("powershell")
            .arg("-Command")
            .arg("Get-ExecutionPolicy")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("Powershell execution policy fetch failed.".to_string()),
            Err(e) => Err(format!("Error running powershell: {}", e)),
        }
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}
pub fn install_scoop_package_manger() -> Result<(), String> {
    if !is_elevated() {
        let status = Command::new("powershell")
            .arg("-Command")
            .arg("Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser; Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("Scoop installation failed.".to_string()),
            Err(e) => Err(format!("Error running scoop: {}", e)),
        }
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}

pub fn install_choco_package_manager() -> Result<(), String> {
    let status = Command::new("powershell")
            .arg("-Command")
            .arg("Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("Choco installation failed.".to_string()),
        Err(e) => Err(format!("Error running choco: {}", e)),
    }
}

pub fn git_clone_repo(repo_url: &str, repo_dir: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(repo_url)
        .arg(repo_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("Git clone failed.".to_string()),
        Err(e) => Err(format!("Error running git: {}", e)),
    }
}
pub fn run_powershell_command(command: &str) -> Result<(), String> {
    let status = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("Powershell command failed.".to_string()),
        Err(e) => Err(format!("Error running powershell: {}", e)),
    }
}

pub fn run_cmd_command(command: &str) -> Result<(), String> {
    let status = Command::new("cmd")
        .arg("/C")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("Command failed.".to_string()),
        Err(e) => Err(format!("Error running command: {}", e)),
    }
}
pub fn is_winget_installed() -> bool {
    Command::new("winget")
        .arg("--version") // A harmless argument to check if winget runs
        .output()
        .is_ok()
}
pub fn is_choco_installed() -> bool {
    Command::new("choco")
        .arg("--version") // A harmless argument to check if choco runs
        .output()
        .is_ok()
}
pub fn is_scoop_installed() -> bool {
    Command::new("scoop")
        .arg("--version") // A harmless argument to check if scoop runs
        .output()
        .is_ok()
}
pub fn install_winget_package(package_name: &str) -> Result<(), String> {
    if is_winget_installed() {
        let status = Command::new("winget")
            .arg("install")
            .arg(package_name)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("Package installation failed.".to_string()),
            Err(e) => Err(format!("Error running winget: {}", e)),
        }
    } else {
        println!("Winget is not installed. Would you like to install it? (y/n)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().eq_ignore_ascii_case("y") {
            let status = Command::new("powershell")
                .arg("-Command")
                .arg("winget install Microsoft.DesktopAppInstaller")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match status {
                Ok(status) if status.success() => Ok(()),
                Ok(_) => Err("Winget installation failed.".to_string()),
                Err(e) => Err(format!("Error running winget: {}", e)),
            }
        } else {
            Err("Winget is not installed.".to_string())
        }
    }
}
pub fn scoop_add_bucket(bucket: ScoopBucket) -> Result<(), String> {
    let bucket_name = match bucket {
        ScoopBucket::Main => "main",
        ScoopBucket::Extras => "extras",
        ScoopBucket::Versions => "versions",
        ScoopBucket::Nirsoft => "nirsoft",
        ScoopBucket::Sysinternals => "sysinternals",
        ScoopBucket::Php => "php",
        ScoopBucket::NerdFonts => "nerd-fonts",
        ScoopBucket::Nonportable => "nonportable",
        ScoopBucket::Java => "java",
        ScoopBucket::Games => "games",
    };

    let status = Command::new("pwsh")
        .arg("-Command")
        .arg("scoop")
        .arg("bucket")
        .arg("add")
        .arg(bucket_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err("Bucket addition failed.".to_string()),
        Err(e) => Err(format!("Error running scoop: {}", e)),
    }
}
pub fn install_scoop_package(package_name: &str) -> Result<(), String> {
    if is_scoop_installed() {
        let status = Command::new("pwsh")
            .arg("-Command")
            .arg("scoop")
            .arg("install")
            .arg(package_name)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(_) => Err("Package installation failed.".to_string()),
            Err(e) => Err(format!("Error running scoop: {}", e)),
        }
    } else {
        println!("Scoop is not installed. Would you like to install it? (y/n)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().eq_ignore_ascii_case("y") {
            install_scoop_package_manger()
        } else {
            Err("Scoop is not installed.".to_string())
        }
    }
}

pub fn install_choco_package(package_name: &str) -> Result<(), String> {
    if is_scoop_installed() {
        if !is_elevated() {
            let status = Command::new("choco")
                .arg("install")
                .arg(package_name)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match status {
                Ok(status) if status.success() => Ok(()),
                Ok(_) => Err("Package installation failed.".to_string()),
                Err(e) => Err(format!("Error running choco: {}", e)),
            }
        } else {
            Err("You are in administrator, it is best to use non-admin priviliges.".to_string())
        }
    } else {
        println!("Winget is not installed. Would you like to install it? (y/n)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim().eq_ignore_ascii_case("y") {
            install_choco_package_manager()
        } else {
            Err("Chocolatey is not installed.".to_string())
        }
    }
}

pub fn install_package(package_name: &str, manager: WinPkgMgrs) -> Result<(), String> {
    match manager {
        WinPkgMgrs::Winget => install_winget_package(package_name),
        WinPkgMgrs::Scoop => install_scoop_package(package_name),
        WinPkgMgrs::Choco => install_choco_package(package_name),
    }
}
