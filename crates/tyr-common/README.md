# tyr-common

Shared infrastructure used throughout the Tyr compiler.

This crate provides:

- Source management
- Source spans
- Diagnostics
- File identifiers
- Common result types

No compiler stage should introduce dependencies into this crate.