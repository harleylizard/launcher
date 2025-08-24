enum Platform {
    Windows,
    Linux,
    Mac
}

#[cfg(target_os = "windows")]
fn get_platform() -> Platform {
    return Platform::Windows
}

#[cfg(target_os = "linux")]
fn get_platform() -> Platform {
    return Platform::Linux;
}

#[cfg(target_os = "macos")]
fn get_platform() -> Platform {
    return Platform::Mac;
}