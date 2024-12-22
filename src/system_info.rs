#[derive(Debug)]
pub struct SystemInfo {
    pub os: String,
    pub package_manager: Option<String>,
    pub gpu: Option<String>,
    pub kernel_version: Option<String>,
}
