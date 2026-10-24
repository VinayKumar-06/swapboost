use std::{
    io,
    mem::{size_of, zeroed},
    time::{SystemTime, UNIX_EPOCH},
};
use swapboost_core::Snapshot;
use windows_sys::Win32::System::ProcessStatus::{GetPerformanceInfo, PERFORMANCE_INFORMATION};
pub fn collect() -> io::Result<Snapshot> {
    // SAFETY: PERFORMANCE_INFORMATION is a C POD structure. The pointer is valid
    // and writable for its full size; the API does not retain it.
    let mut info: PERFORMANCE_INFORMATION = unsafe { zeroed() };
    let size = size_of::<PERFORMANCE_INFORMATION>() as u32;
    if unsafe { GetPerformanceInfo(&mut info, size) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let bytes = |pages: usize| {
        (pages as u64)
            .checked_mul(info.PageSize as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Memory counter overflow"))
    };
    Ok(Snapshot {
        timestamp_unix_ms: SystemTime::now().duration_since(UNIX_EPOCH).map_err(io::Error::other)?.as_millis(),
        platform: "windows".into(), physical_total_bytes: bytes(info.PhysicalTotal)?, physical_available_bytes: Some(bytes(info.PhysicalAvailable)?),
        committed_bytes: Some(bytes(info.CommitTotal)?), commit_limit_bytes: Some(bytes(info.CommitLimit)?),
        limitations: vec!["Direct memory-pressure and paging-rate measurement are not implemented on Windows; pressure is unknown.".into(),"Page-file capacity, usage, configuration and storage headroom are not measured in v0.1. Commit limit is not page-file size.".into()],
        ..Default::default()
    })
}
