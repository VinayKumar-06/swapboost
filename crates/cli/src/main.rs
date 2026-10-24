use clap::{Parser, Subcommand};
use std::{
    io::{self, IsTerminal, Write},
    thread,
    time::{Duration, Instant},
};
use swapboost_core::{Policy, Report};
mod display;
#[derive(Parser)]
#[command(
    version,
    about = "Understand memory pressure before changing anything.",
    arg_required_else_help = true,
    after_help = "Start here: swapboost status\nWatch a workload: swapboost monitor --interval 2 --count 30\nPicture guide: https://github.com/VinayKumar-06/swapboost/blob/main/docs/quickstart.md"
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
fn emit(r: &Report, json: bool) -> io::Result<()> {
    let mut out = io::stdout().lock();
    if json {
        serde_json::to_writer(&mut out, r)?;
        writeln!(out)?;
    } else {
        let color = out.is_terminal()
            && std::env::var_os("NO_COLOR").is_none()
            && std::env::var("TERM").as_deref() != Ok("dumb");
        display::human(&mut out, r, color)?;
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
