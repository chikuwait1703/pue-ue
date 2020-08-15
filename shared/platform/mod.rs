#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub use self::linux::directories;

#[cfg(target_os = "macos")]
pub use self::macos::directories;

#[cfg(target_os = "windows")]
pub use self::windows::directories;
