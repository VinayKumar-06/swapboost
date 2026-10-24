//! Platform-independent measurements and conservative diagnostic policy.
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Snapshot {
    pub timestamp_unix_ms: u128,
    pub platform: String,
    pub physical_total_bytes: u64,
    pub physical_available_bytes: Option<u64>,
    pub committed_bytes: Option<u64>,
    pub commit_limit_bytes: Option<u64>,
    pub swap_total_bytes: Option<u64>,
    pub swap_used_bytes: Option<u64>,
    pub swap_in_pages_total: Option<u64>,
    pub swap_out_pages_total: Option<u64>,
    pub memory_some_stall_percent_avg10: Option<f64>,
    pub zswap_enabled: Option<bool>,
    pub zram_swap_detected: Option<bool>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Pressure {
    Unknown,
    NoElevatedSignal,
    Observing,
    Sustained,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub snapshot: Snapshot,
    pub pressure: Pressure,
    pub explanation: String,
    pub recommendation: String,
    pub changes_applied: bool,
}

/// Time is supplied by the caller from a monotonic clock, not a wall clock.
/// A ten-second gap invalidates continuity. No state grows with sample count.
#[derive(Default)]
pub struct Policy {
    elevated_since: Option<u64>,
    last_ms: Option<u64>,
    sustained: bool,
}
impl Policy {
    pub fn evaluate(&mut self, s: Snapshot, now_ms: u64) -> Report {
        if self
            .last_ms
            .is_some_and(|last| now_ms < last || now_ms - last > 10_000)
        {
            self.elevated_since = None;
            self.sustained = false;
        }
        self.last_ms = Some(now_ms);
        let signal = s
            .memory_some_stall_percent_avg10
            .map(|v| (v >= 5.0, v < 1.0));
        let (pressure, explanation) = match signal {
            None => {
                self.elevated_since = None;
                self.sustained = false;
                (Pressure::Unknown, "No direct pressure measurement is available; RAM utilization alone cannot establish pressure.")
            }
            Some((high, low)) => {
                if low || (!high && !self.sustained) {
                    self.elevated_since = None;
                }
                if low {
                    self.sustained = false;
                }
                if high {
                    let start = *self.elevated_since.get_or_insert(now_ms);
                    if now_ms.saturating_sub(start) >= 30_000 {
                        self.sustained = true;
                    }
                }
                if self.sustained {
                    (Pressure::Sustained, "Memory stall avg10 remained at least 5% across 30 seconds of observations; recovery requires below 1%.")
                } else if high {
                    (Pressure::Observing, "Elevated memory stalls observed; more samples are required to establish sustained pressure.")
                } else {
                    (Pressure::NoElevatedSignal, "No elevated memory stall signal in this observation. This does not guarantee application responsiveness.")
                }
            }
        };
        let recommendation = if pressure == Pressure::Sustained {
            "Inspect workload demand and storage latency before tuning paging. Reduce concurrent memory-heavy work where practical; paging can increase latency."
        } else {
            "Continue observing during the affected workload. No system configuration change is recommended from this evidence."
        };
        Report {
            snapshot: s,
            pressure,
            explanation: explanation.into(),
            recommendation: recommendation.into(),
            changes_applied: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample(p: Option<f64>) -> Snapshot {
        Snapshot {
            memory_some_stall_percent_avg10: p,
            ..Default::default()
        }
    }
    #[test]
    fn high_usage_is_not_pressure() {
        assert_eq!(
            Policy::default().evaluate(sample(None), 0).pressure,
            Pressure::Unknown
        );
    }
    #[test]
    fn sustained_and_hysteresis() {
        let mut p = Policy::default();
        for t in (0..30_000).step_by(5000) {
            assert_eq!(
                p.evaluate(sample(Some(6.0)), t).pressure,
                Pressure::Observing
            );
        }
        assert_eq!(
            p.evaluate(sample(Some(6.0)), 30_000).pressure,
            Pressure::Sustained
        );
        assert_eq!(
            p.evaluate(sample(Some(2.0)), 35_000).pressure,
            Pressure::Sustained
        );
        assert_eq!(
            p.evaluate(sample(Some(0.5)), 40_000).pressure,
            Pressure::NoElevatedSignal
        );
    }
    #[test]
    fn gaps_and_missing_data_reset() {
        let mut p = Policy::default();
        p.evaluate(sample(Some(6.0)), 0);
        assert_eq!(
            p.evaluate(sample(Some(6.0)), 31_000).pressure,
            Pressure::Observing
        );
        p.evaluate(sample(None), 32_000);
        assert_eq!(
            p.evaluate(sample(Some(6.0)), 33_000).pressure,
            Pressure::Observing
        );
    }
    #[test]
    fn brief_spike_does_not_trigger() {
        let mut p = Policy::default();
        p.evaluate(sample(Some(8.0)), 0);
        assert_eq!(
            p.evaluate(sample(Some(0.0)), 5000).pressure,
            Pressure::NoElevatedSignal
        );
    }
}
