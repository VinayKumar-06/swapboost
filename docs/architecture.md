# Architecture

The Cargo workspace has three crates: `swapboost-core` owns serializable measurements and a bounded deterministic policy; `swapboost-platform` reads native metrics; `swapboost` parses CLI arguments and emits reports. Dependencies flow from CLI to platform/core, and platform to core. The policy never calls an OS API.

Required metric failures return errors. Optional failures produce absent values and known limitations. PSI state uses monotonic elapsed time; timestamps use Unix milliseconds for report correlation. No sample history, process data, settings, or persistent state are retained. Windows FFI is limited to the documented GetPerformanceInfo call with a correctly sized writable structure.

Portable parser/policy tests execute everywhere. Native smoke tests verify positive physical RAM and valid JSON on each target OS. Formatting, lint, documentation, and release compilation are CI gates. Publication is independent of whether every workflow has completed; consult the actual CI status.
