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
    if !is_elevated() {
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
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}

pub fn git_clone_repo(repo_url: &str, repo_dir: &str) -> Result<(), String> {
    if !is_elevated() {
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
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}
pub fn run_powershell_command(command: &str) -> Result<(), String> {
    if !is_elevated() {
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
    } else {
        Err("Please rerun this program as Administrator.".to_string())
    }
}
pub fn install_winget_package(package_name: &str) -> Result<(), String> {
    if is_elevated() {
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
        Err("Please rerun this program as Administrator.".to_string())
    }
}
pub fn scoop_add_bucket(bucket: ScoopBucket) -> Result<(), String> {
    if !is_elevated() {
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
    } else {
        Err("You are in administrator, it is best to use non-admin priviliges. Exiting".to_string())
    }
}
pub fn install_scoop_package(package_name: &str) -> Result<(), String> {
    if !is_elevated() {
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
        Err("You are in administrator, it is best to use non-admin priviliges. Exiting".to_string())
    }
}

pub fn install_choco_package(package_name: &str) -> Result<(), String> {
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
}

pub fn install_package(package_name: &str, manager: WinPkgMgrs) -> Result<(), String> {
    match manager {
        WinPkgMgrs::Winget => install_winget_package(package_name),
        WinPkgMgrs::Scoop => install_scoop_package(package_name),
        WinPkgMgrs::Choco => install_choco_package(package_name),
    }
}
