use clap::{Parser, Subcommand};
use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};
use swapboost_core::{Policy, Report};
#[derive(Parser)]
#[command(
    version,
    about = "Read-only memory diagnostics. Never modifies OS settings."
)]
struct Args {
    /// Emit JSON (one report per line when monitoring).
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Report current memory metrics.
    Status,
    /// Explain one observation. Use monitor to establish sustained pressure.
    Diagnose,
    /// Give a conservative recommendation from one observation.
    Recommend,
    /// Sample repeatedly; Ctrl+C stops. Memory use is bounded.
    Monitor {
        /// Seconds between samples (1 through 10).
        #[arg(long,default_value_t=2,value_parser=clap::value_parser!(u64).range(1..=10))]
        interval: u64,
        /// Stop after this many samples (1 through 86400).
        #[arg(long,default_value_t=30,value_parser=clap::value_parser!(u64).range(1..=86400))]
        count: u64,
    },
}
fn memory(v: Option<u64>) -> String {
    v.map(|b| format!("{:.2} GiB", b as f64 / 1_073_741_824.0))
        .unwrap_or_else(|| "unavailable".into())
}
fn emit(r: &Report, json: bool) -> io::Result<()> {
    let mut out = io::stdout().lock();
    if json {
        serde_json::to_writer(&mut out, r)?;
        writeln!(out)?;
    } else {
        let s = &r.snapshot;
        writeln!(out,"SwapBoost 0.1 | {} | {} ms since Unix epoch\nPhysical RAM: {}\nAvailable RAM: {}\nCommitted: {} / {}\nSwap: {} used / {} capacity\nPressure: {:?}\n{}\nRecommendation: {}",s.platform,s.timestamp_unix_ms,memory(Some(s.physical_total_bytes)),memory(s.physical_available_bytes),memory(s.committed_bytes),memory(s.commit_limit_bytes),memory(s.swap_used_bytes),memory(s.swap_total_bytes),r.pressure,r.explanation,r.recommendation)?;
        writeln!(out,"Memory stall avg10: {:?}% | Swap-in/out pages since boot: {:?}/{:?}\nZswap enabled: {:?} | Zram swap detected: {:?}",s.memory_some_stall_percent_avg10,s.swap_in_pages_total,s.swap_out_pages_total,s.zswap_enabled,s.zram_swap_detected)?;
        for limit in &s.limitations {
            writeln!(out, "Note: {limit}")?;
        }
        writeln!(out, "Read-only: no changes applied.\n")?;
    }
    out.flush()
}
fn run(args: Args) -> io::Result<()> {
    let (count, interval) = match args.command {
        Command::Monitor { count, interval } => (count, interval),
        _ => (1, 0),
    };
    let mut policy = Policy::default();
    let start = Instant::now();
    for i in 0..count {
        let report = policy.evaluate(
            swapboost_platform::collect()?,
            start.elapsed().as_millis() as u64,
        );
        emit(&report, args.json)?;
        if i + 1 < count {
            thread::sleep(Duration::from_secs(interval));
        }
    }
    Ok(())
}
fn main() {
    if let Err(error) = run(Args::parse()) {
        if error.kind() == io::ErrorKind::BrokenPipe {
            return;
        }
        eprintln!("swapboost: {error}");
        std::process::exit(1);
    }
}
