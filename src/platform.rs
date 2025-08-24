use std::mem::discriminant;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum Platform {
    Windows,
    Linux,
    Mac
}

#[cfg(target_os = "windows")]
pub(crate) fn get_platform() -> Platform {
    Platform::Windows
}

#[cfg(target_os = "linux")]
pub(crate) fn get_platform() -> Platform {
    Platform::Linux;
}

#[cfg(target_os = "macos")]
pub(crate) fn get_platform() -> Platform {
    Platform::Mac;
}

impl PartialEq<Platform> for &Platform {

    fn eq(&self, other: &Platform) -> bool {
        discriminant(*self) == discriminant(other)
    }
}