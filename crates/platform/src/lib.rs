//! Read-only native adapters. Missing optional measurements stay unknown.
pub mod linux;
#[cfg(windows)]
mod windows;
use std::io;
use swapboost_core::Snapshot;

pub fn collect() -> io::Result<Snapshot> {
    #[cfg(target_os = "linux")]
    {
        linux::collect()
    }
    #[cfg(windows)]
    {
        windows::collect()
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        Err(io::Error::new(io::ErrorKind::Unsupported, "SwapBoost currently collects metrics on Windows and Linux only. macOS is not supported."))
    }
}
