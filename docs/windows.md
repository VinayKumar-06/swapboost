# Windows

The native adapter uses windows-sys bindings to GetPerformanceInfo for physical and committed memory. It does not launch PowerShell, WMI scripts, or subprocesses. API failures return the Windows error.

Page-file configuration, page-file usage, compression, paging rates, and direct pressure signals are not yet implemented. Unsupported values are null. No page-file changes or reboot operations exist. Use maintained Windows versions; the CI Windows runner exercises native collection, but does not establish compatibility with every desktop version.
