# Security

Version 0.1 is an early read-only release. Keep to the latest repository revision while development continues. No support SLA or security audit is claimed.

For a vulnerability, use GitHub's private vulnerability reporting feature if it is enabled. Otherwise open an issue requesting a private reporting channel without publishing exploit details or sensitive data. Do not include credentials, private process details, or unredacted machine reports.

The runtime performs no network requests, command execution, privileged changes, or process enumeration. Dependency downloads and GitHub CI have separate network access. See docs/security.md for the threat model.
