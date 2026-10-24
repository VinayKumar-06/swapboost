use std::io::{self, Write};
use swapboost_core::{Pressure, Report};
fn size(value: Option<u64>) -> String {
    value
        .map(|b| format!("{:.2} GiB", b as f64 / 1_073_741_824.0))
        .unwrap_or_else(|| "Not available".into())
}
fn number(value: Option<u64>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "Not available".into())
}
fn yes_no(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "Yes",
        Some(false) => "No",
        None => "Not available",
    }
}
fn paragraph(out: &mut impl Write, text: &str) -> io::Result<()> {
    let mut line = String::from("  ");
    for word in text.split_whitespace() {
        if line.len() + word.len() + 1 > 72 {
            writeln!(out, "{}", line.trim_end())?;
            line = String::from("  ");
        }
        line.push_str(word);
        line.push(' ');
    }
    writeln!(out, "{}", line.trim_end())
}
pub fn human(out: &mut impl Write, report: &Report, color: bool) -> io::Result<()> {
    let s = &report.snapshot;
    let (accent, reset) = if color {
        ("\x1b[1;36m", "\x1b[0m")
    } else {
        ("", "")
    };
    writeln!(
        out,
        "\n{accent}  SWAPBOOST  {}{reset}  |  {}  |  READ ONLY",
        env!("CARGO_PKG_VERSION"),
        s.platform
    )?;
    writeln!(
        out,
        "  --------------------------------------------------------------------"
    )?;
    let state = match report.pressure {
        Pressure::Unknown => "NOT MEASURED",
        Pressure::NoElevatedSignal => "NO ELEVATED SIGNAL",
        Pressure::Observing => "OBSERVING",
        Pressure::Sustained => "SUSTAINED PRESSURE",
    };
    writeln!(out, "\n  MEMORY PRESSURE   {state}")?;
    paragraph(out, &report.explanation)?;
    writeln!(out, "\n  MEMORY AT A GLANCE")?;
    writeln!(
        out,
        "  {:<25} {}",
        "Physical RAM",
        size(Some(s.physical_total_bytes))
    )?;
    writeln!(
        out,
        "  {:<25} {}",
        "Available RAM",
        size(s.physical_available_bytes)
    )?;
    if let Some(available) = s
        .physical_available_bytes
        .filter(|v| s.physical_total_bytes > 0 && *v <= s.physical_total_bytes)
    {
        let fraction = available as f64 / s.physical_total_bytes as f64;
        let bars = (fraction * 20.0).round() as usize;
        writeln!(
            out,
            "  Available [{}{}] {:>5.1}%",
            "#".repeat(bars),
            "-".repeat(20 - bars),
            fraction * 100.0
        )?;
        writeln!(
            out,
            "  Capacity indicator only; high use does not prove pressure."
        )?;
    }
    writeln!(
        out,
        "  {:<25} {} / {}",
        "Commit / limit",
        size(s.committed_bytes),
        size(s.commit_limit_bytes)
    )?;
    writeln!(
        out,
        "  {:<25} {} / {}",
        "Swap used / capacity",
        size(s.swap_used_bytes),
        size(s.swap_total_bytes)
    )?;
    writeln!(out, "\n  WHAT TO DO NEXT")?;
    paragraph(out, &report.recommendation)?;
    writeln!(out, "\n  DETAILS")?;
    let psi = s
        .memory_some_stall_percent_avg10
        .map(|v| format!("{v:.2}%"))
        .unwrap_or_else(|| "Not available".into());
    writeln!(out,"  Memory stalls (avg10)      {psi}\n  Swap-in pages since boot   {}\n  Swap-out pages since boot  {}\n  Zswap enabled              {}\n  Zram swap detected         {}",number(s.swap_in_pages_total),number(s.swap_out_pages_total),yes_no(s.zswap_enabled),yes_no(s.zram_swap_detected))?;
    if !s.limitations.is_empty() {
        writeln!(out, "\n  MEASUREMENT LIMITS")?;
        for note in &s.limitations {
            paragraph(out, &format!("- {note}"))?;
        }
    }
    writeln!(
        out,
        "\n  No settings changed. No data uploaded.\n  Sample time: {} ms since Unix epoch\n",
        s.timestamp_unix_ms
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use swapboost_core::{Policy, Snapshot};
    #[test]
    fn unavailable_is_not_zero_and_pipe_has_no_ansi() {
        let r = Policy::default().evaluate(
            Snapshot {
                platform: "windows".into(),
                physical_total_bytes: 1024,
                ..Default::default()
            },
            0,
        );
        let mut output = Vec::new();
        human(&mut output, &r, false).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("NOT MEASURED"));
        assert!(text.contains("Not available"));
        assert!(!text.contains("Some("));
        assert!(!text.contains('\x1b'));
        assert!(!text.contains("Available ["));
    }
    #[test]
    fn invalid_available_does_not_underflow_bar() {
        let r = Policy::default().evaluate(
            Snapshot {
                physical_total_bytes: 1,
                physical_available_bytes: Some(2),
                ..Default::default()
            },
            0,
        );
        let mut output = Vec::new();
        human(&mut output, &r, false).unwrap();
        assert!(!String::from_utf8(output).unwrap().contains("Available ["));
    }
}
