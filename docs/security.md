# Threat model

The monitor runs as an ordinary user and reads fixed OS metric paths/native counters. It neither opens user-supplied configuration paths nor constructs shell commands. CLI intervals and counts are bounded. Read errors and malformed required values fail explicitly; optional values remain unknown. There is no privileged helper or mutation endpoint.

OS metric sources are trusted kernel interfaces; these are not general-purpose parsers for arbitrary unbounded user files. Finite monitoring avoids unbounded history but output redirected to disk can still accumulate. Dependencies and build tools have their own supply-chain risks. The repository commits Cargo.lock; use --locked for builds. Do not run unknown builds as administrator.
