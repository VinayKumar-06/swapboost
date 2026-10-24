# SwapBoost

![SwapBoost: understand memory pressure before changing anything](docs/images/hero.svg)

**Understand memory pressure before changing anything.**

[![CI](https://github.com/VinayKumar-06/swapboost/actions/workflows/ci.yml/badge.svg)](https://github.com/VinayKumar-06/swapboost/actions/workflows/ci.yml)

SwapBoost is an MIT-licensed, local, read-only memory diagnostic CLI for Windows and Linux. Version 0.1 is an early foundation, not an automatic optimizer or a performance guarantee. It does not create physical RAM, clear caches, allocate swap, edit settings, or require administrator access.

## Start here

**[Download v0.1 preview](https://github.com/VinayKumar-06/swapboost/releases/tag/v0.1.0)** · **[Picture-by-picture setup guide](docs/quickstart.md)** · **[Report a bug](https://github.com/VinayKumar-06/swapboost/issues/new?template=bug_report.yml)**

| Windows (Intel/AMD 64-bit) | Linux (Intel/AMD 64-bit) |
| --- | --- |
| [Download ZIP](https://github.com/VinayKumar-06/swapboost/releases/download/v0.1.0/swapboost-windows-x86_64.zip) | [Download tar.gz](https://github.com/VinayKumar-06/swapboost/releases/download/v0.1.0/swapboost-linux-x86_64.tar.gz) |
| Extract → open PowerShell in the folder | Extract → open a terminal in the folder |
| `.\swapboost.exe status` | `./swapboost status` |

No installer, administrator access, or Rust compiler is required for these downloads. This is a **terminal app**: run it from PowerShell or a terminal instead of double-clicking. The preview is unsigned. Linux builds target the Ubuntu runner's libraries; older distributions may need a source build. macOS monitoring and ARM prebuilt downloads are not supported.

![Four example readings with guidance: pressure is distinct from available RAM. Illustration, not live data.](docs/images/04-read.svg)

### Why try it?

- **Understand first:** see available RAM, commit, and supported paging metrics together.
- **Observe safely:** no cache clearing, configuration changes, or network telemetry.
- **Know what is missing:** unsupported measurements are clearly labeled.
- **Use your own workflow:** readable terminal reports or JSON for scripts.

### Build from source

Install [Rust stable](https://rustup.rs/) and Git, then:

```sh
git clone https://github.com/VinayKumar-06/swapboost.git
cd swapboost
cargo install --locked --path crates/cli
swapboost status
swapboost monitor --interval 2 --count 30
```

Windows builds may require Microsoft's C++ build tools. If `swapboost` is not found after installation, reopen the terminal or run `cargo run -p swapboost -- status` from the repository.

## Commands

| Command | Behavior |
| --- | --- |
| `status` | One current report |
| `monitor --interval 2 --count 30` | Finite sampling, retaining only policy state |
| `diagnose` | Explain one current observation |
| `recommend` | Conservative advice from one observation |
| `--json` | Machine-readable report; monitoring emits newline-delimited JSON |
| `--help` | Command and option documentation |

`status`, `diagnose`, and `recommend` currently share one report format. A single sample cannot establish sustained pressure. Stop monitoring with Ctrl+C. Intervals are restricted to 1–10 seconds and counts to 1–86400. No background service is installed. Uninstall with `cargo uninstall swapboost`, or delete an extracted executable.

## Capabilities

| Measurement | Linux | Windows |
| --- | --- | --- |
| Physical and available RAM | `/proc/meminfo` | `GetPerformanceInfo` |
| Commit / limit | Linux accounting, separately labeled | Native commit counters |
| Swap capacity / usage | Yes, when exposed | Not implemented |
| Swap-in/out cumulative pages | `/proc/vmstat` | Not implemented |
| Direct memory pressure | PSI if available | Not implemented; unknown |
| zram / zswap detection | Best effort | Not implemented |
| Storage headroom and configuration changes | Not implemented | Not implemented |

macOS is not a supported monitoring platform. It can run portable tests and reports an explicit error for collection commands. Target deployments are maintained 64-bit Windows and Linux systems; CI is a smoke test, not certification of every OS version. Containers report host counters and may have much smaller cgroup limits.

## Interpreting results

High RAM utilization is not sufficient evidence of pressure. Linux diagnosis observes PSI `some avg10`: at least 5% across 30 seconds of consecutive observations establishes sustained pressure. Below 1% clears it; sample gaps over ten seconds reset continuity. These are conservative initial heuristics, not validated universal performance thresholds. The Windows adapter deliberately leaves pressure unknown until a direct signal is implemented.

All sizes in JSON are bytes; display sizes are GiB. Paging counters are cumulative pages since boot, not rates or bytes. Unavailable metrics are `null`, never fabricated zeros. Commit, swap, and physical RAM are different quantities and are never added into a misleading RAM total. Linux overcommit accounting differs from Windows commit. Paging may accommodate more workloads but can cause latency and storage writes.

## Safety and privacy

No system changes, privilege escalation, network telemetry, process inspection, workload generation, or automatic tuning are implemented. Reports remain on standard output unless you redirect them. Reports still reveal hardware capacity and workload pressure; inspect them before sharing. The software can contain bugs and has monitoring overhead; it cannot promise zero risk, prevent OOM failures, or guarantee faster applications. There is no rollback because this version has no configuration mutation path.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo doc --locked --workspace --no-deps
```

CI runs these checks plus actual OS collection smoke tests on Linux and Windows. See [architecture](docs/architecture.md), [memory concepts](docs/memory-management.md), [benchmarking](docs/benchmarking.md), [contributing](CONTRIBUTING.md), and [security](SECURITY.md).

## Roadmap

- Implement Windows pressure, paging, and page-file configuration inspection.
- Add Linux cgroup awareness, paging rates, and storage-headroom inspection.
- Validate richer recommendations against isolated, reproducible workloads.
- Introduce dry-run configuration plans, then individually reviewed operations with backups and rollback.
- Consider opt-in adaptation only after comparative evidence and failure testing.

Configuration apply/rollback, stress benchmarks, GUI, automatic optimization, and signed releases are **not implemented**. No benchmark improvements are claimed.

## Help shape SwapBoost

Try it during a normal workload, then [report a reproducible issue](https://github.com/VinayKumar-06/swapboost/issues/new?template=bug_report.yml) or [suggest an improvement](https://github.com/VinayKumar-06/swapboost/issues/new?template=feature_request.yml). Documentation fixes, accessibility feedback, and platform testing are welcome. See [Contributing](CONTRIBUTING.md).

If it is useful, a star helps you find the project again, and sharing the repository with someone who needs memory diagnostics helps it reach the right users. Please describe it as a read-only diagnostic preview, not a RAM booster. [Ready-to-use project introduction](docs/share.md).
