#[cfg(windows)]
pub const WINDOWS: bool = true;
#[cfg(not(windows))]
pub const WINDOWS: bool = false;
#[cfg_attr(windows, cfg(not(target_pointer_width = "64")))]
compile_error!("64-bit is required on windows!");

#[cfg(target_os = "linux")]
pub const LINUX: bool = true;
#[cfg(not(target_os = "linux"))]
pub const LINUX: bool = false;

#[cfg(target_os = "android")]
pub const ANDROID: bool = true;
#[cfg(not(target_os = "android"))]
pub const ANDROID: bool = false;

#[cfg(unix)]
pub const UNIX: bool = true;
#[cfg(not(unix))]
pub const UNIX: bool = false;

#[cfg(target_vendor = "apple")]
pub const APPLE: bool = true;
#[cfg(not(target_vendor = "apple"))]
pub const APPLE: bool = false;

#[cfg(target_os = "ios")]
pub const IOS: bool = true;
#[cfg(not(target_os = "ios"))]
pub const IOS: bool = false;

#[cfg(target_os = "macos")]
pub const MACOS: bool = true;
#[cfg(not(target_os = "macos"))]
pub const MACOS: bool = false;

#[cfg(all(
    target_vendor = "apple",
    not(any(target_os = "ios", target_os = "macos"))
))]
compile_error!("Unknown Apple platform");

#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "android",
    target_os = "ios",
    target_os = "macos"
)))]
compile_error!("Unknow platform!");
