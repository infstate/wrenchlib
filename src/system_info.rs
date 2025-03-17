use crate::os::WinPkgMgrs;

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct SystemInfo {
    pub package_manager: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Debug)]
pub struct SystemInfo {
    pub package_manager: Option<WinPkgMgrs>,
}
