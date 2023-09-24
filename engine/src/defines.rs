pub use cfg_if::cfg_if;

// Platform detection
cfg_if! {
    if #[cfg(windows)] {
        pub const WINDOWS: bool = true;
        #[cfg(not(target_pointer_width = "64"))]
        compile_error!("64-bit is required on Windows!");
    } else if #[cfg(target_os = "linux")]{
        pub const LINUX: bool = true;
        #[cfg(target_os = "android")]
        pub const ANDROID: bool = true;
    } else if #[cfg(unix)]{
        pub const UNIX: bool = true;
    // } else if #[cfg(target_os = "posix")]{
    //     pub const POSIX: bool = true;
    } else if #[cfg(target_vendor = "apple")]{
        pub const APPLE: bool = true;
        cfg_if! {
            if #[cfg(ios simulator)] {
                pub const IOS: bool = true;
                pub const IOS_SIMULATOR: bool = true;
            } else if #[cfg(target_os = "ios")] {
                pub const IOS: bool = true;
            }
        }
    }
}

#[cfg(target_vendor = "apple")]
// Apple platform
pub const APPLE: bool = true;
#[cfg(not(target_vendor = "apple"))]
pub const APPLE: bool = false;

#[cfg(target_os = "ios")]
// iOS device
pub const IOS: bool = true;
#[cfg(not(target_os = "ios"))]
pub const IOS: bool = false;

#[cfg(target_os = "macos")]
// Other kinds of Mac OS
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
