use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum Platform {
    Windows,
    Linux,
    Mac
}

#[cfg(target_os = "windows")]
fn get_platform() -> Platform {
    Platform::Windows
}

#[cfg(target_os = "linux")]
fn get_platform() -> Platform {
    Platform::Linux;
}

#[cfg(target_os = "macos")]
fn get_platform() -> Platform {
    Platform::Mac;
}