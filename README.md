# tyrc

<div align="center">

<img src="https://raw.githubusercontent.com/thetyrproject/branding/main/tyr-logo-black.png" alt="Tyr Logo" width="180"/>

### The Reference Compiler for the Tyr Hardware Description Language

**One Language. Every Digital Architecture.**

</div>

---

## Overview

**tyrc** is the reference compiler for the **Tyr Hardware Description Language**.

It implements the official Tyr Language Specification and translates Tyr hardware descriptions into backend-specific representations while preserving the semantics defined by the language.

The compiler is written in **Rust** and follows a modular, multi-crate architecture to encourage maintainability, correctness, and long-term evolution.

---

## Project Goals

- Complete implementation of the Tyr Language Specification
- Modular compiler architecture
- Modern diagnostics
- Strict language compliance
- Backend-independent compilation
- Extensible compiler infrastructure

---

## Workspace Layout

```
tyrc/

├── crates/
│   ├── tyr-common/
│   ├── tyr-lexer/
│   ├── tyr-parser/
│   ├── tyr-ast/
│   ├── tyr-sema/
│   ├── tyr-hir/
│   ├── tyr-backend/
│   ├── tyr-driver/
│   └── tyrc/
│
├── docs/
├── examples/
├── tests/
└── .github/
```

---

## Compiler Pipeline

```
Source File
     │
     ▼
Compiler Directives
     │
     ▼
Lexer
     │
     ▼
Parser
     │
     ▼
Abstract Syntax Tree (AST)
     │
     ▼
Semantic Analysis
     │
     ▼
Hardware Intermediate Representation (HIR)
     │
     ▼
Hardware Graph
     │
     ▼
Optimization
     │
     ▼
Backend
```

---

## Current Status

The project is currently in active development.

### Completed

- Workspace architecture
- Common infrastructure (`tyr-common`)
- Lexical analyzer (`tyr-lexer`)
- GitHub Actions CI
- Unit tests
- Formatting and Clippy compliance

### In Progress

- Recursive-descent parser
- AST construction

### Planned

- Semantic analysis
- HIR generation
- Backend
- Verilog/SystemVerilog emitter
- GTKWave-compatible waveform generation
- VSCode extension

---

## Building

Requirements:

- Rust (latest stable)
- Cargo

Build:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Lint:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Format:

```bash
cargo fmt --all
```

---

## Contributing

Contributions are welcome.

Please ensure that every contribution:

- compiles successfully
- passes formatting
- passes Clippy
- passes all tests

Compiler behaviour shall remain consistent with the Tyr Language Specification.

---

## Related Projects

| Repository | Description |
|------------|-------------|
| tyr | Language Specification |
| tyr-rfcs | Language Evolution |
| tyr-book | Official Documentation |
| branding | Branding Assets |
| governance | Project Governance |

---

## License

Licensed under the Apache License, Version 2.0.

---

<div align="center">

**The Tyr Project**

One Language. Every Digital Architecture.

</div>