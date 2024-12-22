use crate::system_info::SystemInfo;

pub fn collect_system_info() -> SystemInfo {
    #[cfg(target_os = "linux")]
    {
        crate::os::linux::collect_system_info()
    }

    #[cfg(target_os = "windows")]
    {
        crate::os::windows::collect_system_info()
    }
}
