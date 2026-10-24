# Memory measurements

Physical RAM is hardware. Available RAM includes memory the OS can readily reuse; cached memory is not automatically wasted memory. Commit accounting describes promises/accounting of memory and differs by OS. Swap/page files are backing stores, not extra physical RAM. Compressed memory uses CPU and RAM to reduce the physical footprint of some pages.

Linux uses procfs counters and optionally Pressure Stall Information. `some avg10` measures time when at least some tasks stalled on memory, averaged over ten seconds. Kernel sources: https://www.kernel.org/doc/html/latest/accounting/psi.html and https://www.kernel.org/doc/html/latest/filesystems/proc.html .

Windows uses the documented PERFORMANCE_INFORMATION page counters multiplied by PageSize: https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-performance_information . CommitLimit must not be relabeled as page-file capacity.
