use std::{
    collections::BTreeMap,
    fs, io,
    time::{SystemTime, UNIX_EPOCH},
};
use swapboost_core::Snapshot;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

pub fn parse_meminfo(text: &str) -> io::Result<BTreeMap<String, u64>> {
    let mut values = BTreeMap::new();
    for line in text.lines() {
        let Some((key, rest)) = line.split_once(':') else {
            continue;
        };
        if ![
            "MemTotal",
            "MemAvailable",
            "SwapTotal",
            "SwapFree",
            "Committed_AS",
            "CommitLimit",
        ]
        .contains(&key)
        {
            continue;
        }
        let mut fields = rest.split_whitespace();
        let value = fields
            .next()
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or_else(|| invalid("Invalid memory value"))?;
        if fields.next() != Some("kB") || fields.next().is_some() {
            return Err(invalid("Unexpected memory unit"));
        }
        let bytes = value
            .checked_mul(1024)
            .ok_or_else(|| invalid("Memory counter overflow"))?;
        if values.insert(key.into(), bytes).is_some() {
            return Err(invalid("Duplicate memory counter"));
        }
    }
    let total = *values
        .get("MemTotal")
        .ok_or_else(|| invalid("MemTotal missing"))?;
    if total == 0 || values.get("MemAvailable").is_some_and(|a| *a > total) {
        return Err(invalid("Invalid physical memory counters"));
    }
    if let (Some(total), Some(free)) = (values.get("SwapTotal"), values.get("SwapFree")) {
        if free > total {
            return Err(invalid("Invalid swap counters"));
        }
    }
    Ok(values)
}
pub fn parse_psi(text: &str) -> Option<f64> {
    let line = text
        .lines()
        .find(|l| l.split_whitespace().next() == Some("some"))?;
    let value: f64 = line
        .split_whitespace()
        .find_map(|s| s.strip_prefix("avg10="))?
        .parse()
        .ok()?;
    (value.is_finite() && (0.0..=100.0).contains(&value)).then_some(value)
}
pub fn counter(text: &str, name: &str) -> Option<u64> {
    text.lines().find_map(|l| {
        let mut f = l.split_whitespace();
        if f.next()? != name {
            return None;
        }
        f.next()?.parse().ok()
    })
}
pub fn collect() -> io::Result<Snapshot> {
    let m = parse_meminfo(&fs::read_to_string("/proc/meminfo")?)?;
    let vm = fs::read_to_string("/proc/vmstat").ok();
    let psi = fs::read_to_string("/proc/pressure/memory")
        .ok()
        .and_then(|v| parse_psi(&v));
    let swaps = fs::read_to_string("/proc/swaps").ok();
    let zswap = fs::read_to_string("/sys/module/zswap/parameters/enabled")
        .ok()
        .and_then(|s| match s.trim() {
            "Y" | "1" => Some(true),
            "N" | "0" => Some(false),
            _ => None,
        });
    let mut limitations = vec![
        "Host-wide counters; container/cgroup limits are not evaluated.".into(),
        "Storage headroom and swap configuration changes are not supported in v0.1.".into(),
        "Linux commit accounting is not equivalent to Windows commit accounting.".into(),
    ];
    if psi.is_none() {
        limitations.push("Memory PSI unavailable or malformed; pressure remains unknown.".into());
    }
    Ok(Snapshot {
        timestamp_unix_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_millis(),
        platform: "linux".into(),
        physical_total_bytes: m["MemTotal"],
        physical_available_bytes: m.get("MemAvailable").copied(),
        committed_bytes: m.get("Committed_AS").copied(),
        commit_limit_bytes: m.get("CommitLimit").copied(),
        swap_total_bytes: m.get("SwapTotal").copied(),
        swap_used_bytes: m
            .get("SwapTotal")
            .zip(m.get("SwapFree"))
            .and_then(|(t, f)| t.checked_sub(*f)),
        swap_in_pages_total: vm.as_deref().and_then(|v| counter(v, "pswpin")),
        swap_out_pages_total: vm.as_deref().and_then(|v| counter(v, "pswpout")),
        memory_some_stall_percent_avg10: psi,
        zswap_enabled: zswap,
        zram_swap_detected: swaps.map(|s| {
            s.lines().skip(1).any(|l| {
                l.split_whitespace()
                    .next()
                    .is_some_and(|p| p.starts_with("/dev/zram"))
            })
        }),
        limitations,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn converts_kib_to_bytes() {
        assert_eq!(
            parse_meminfo("MemTotal: 100 kB\nMemAvailable: 20 kB").unwrap()["MemAvailable"],
            20480
        );
    }
    #[test]
    fn rejects_invalid_metrics() {
        for input in [
            "",
            "MemTotal: 0 kB",
            "MemTotal: 1 MB",
            "MemTotal: 1 kB\nMemAvailable: 2 kB",
            "MemTotal: 18446744073709551615 kB",
            "MemTotal: 1 kB\nMemTotal: 2 kB",
            "MemTotal: 1 kB\nSwapTotal: 1 kB\nSwapFree: 2 kB",
        ] {
            assert!(parse_meminfo(input).is_err(), "{input}");
        }
    }
    #[test]
    fn optional_metrics_stay_missing() {
        assert!(!parse_meminfo("MemTotal: 100 kB")
            .unwrap()
            .contains_key("MemAvailable"));
    }
    #[test]
    fn psi_validation() {
        assert_eq!(parse_psi("some avg10=2.50 avg60=1.0"), Some(2.5));
        for s in [
            "",
            "some avg10=NaN",
            "some avg10=101",
            "full avg10=3",
            "some avg10=-1",
        ] {
            assert_eq!(parse_psi(s), None);
        }
    }
    #[test]
    fn vm_counters() {
        assert_eq!(counter("pswpin 42\npswpout 9", "pswpin"), Some(42));
        assert_eq!(counter("pswpin bad", "pswpin"), None);
    }
}
