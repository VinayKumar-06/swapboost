# Benchmark methodology (future work)

No stress workload generator or comparative benchmark results are included in v0.1. `monitor` measures existing activity only. It is not a benchmark command and does not measure storage latency or application responsiveness.

Future experiments must use disposable VMs with explicit RAM/disk limits, fixed OS versions, repeated baseline/treatment trials, and one setting changed at a time. Record workload completion, failures, pressure duration, paging, storage latency, CPU cost, and responsiveness. Include neutral results and regressions; never deliberately exhaust a primary machine or a shared CI runner.
