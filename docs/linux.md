# Linux

Collection reads `/proc/meminfo`, `/proc/vmstat`, `/proc/pressure/memory`, `/proc/swaps`, and `/sys/module/zswap/parameters/enabled`. The process needs read permission only. Missing PSI produces unknown pressure. Missing optional counters remain absent. zram detection checks active swap device names, so aliases can escape detection; zswap enabled does not imply that pages are compressed.

Measurements are host-wide and are not a container memory-budget check. Restricted procfs mounts may prevent monitoring. No swappiness, swap, OOM, filesystem, service, or cgroup setting is changed. Storage capacity and distribution service ownership are not yet inspected, so no allocation advice is generated.
