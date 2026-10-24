# Contributing

Issues and pull requests are welcome. Describe the OS, SwapBoost version, expected behavior, actual behavior, and a minimal reproduction. Review logs for private information before sharing. Be respectful and focus on reproducible evidence.

Run the four development checks in README before submitting. Add tests for behavior changes, including malformed/missing metrics and capability boundaries. Keep platform code outside the policy crate. Native integration tests must remain read-only; never stress shared runners or require administrator access.

New configuration operations require a separate design review covering permissions, validation, backup, partial failures, verification, rollback, and supported OS versions. Do not add shell-based tuning or unverified performance claims. Contributions are licensed under the repository's MIT license.
