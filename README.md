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

**Current Release:** **v0.2.0**

The Tyr compiler (`tyrc`) is under active development. The project currently implements the initial frontend infrastructure, including lexical analysis, parsing and AST construction.

### Implemented

| Component | Status | Notes |
|-----------|:------:|-------|
| Workspace | ✅ | Cargo workspace with modular crate architecture |
| CI/CD | ✅ | GitHub Actions (fmt, check, clippy, tests) |
| `tyr-common` | ✅ | Diagnostics, spans, source management and common utilities |
| `tyr-lexer` | ✅ | UTF-8 lexer with keywords, literals, operators, punctuation, comments and diagnostics |
| `tyr-ast` | ✅ | Core AST nodes (`CompilationUnit`, `Module`, `Identifier`, `Item`) |
| `tyr-parser` | 🟡 | Initial recursive-descent parser (Compilation Units and Modules) |
| Unit Tests | ✅ | Workspace-wide tests for implemented crates |

### In Progress

- Parsing module members
- Signal declarations
- Register declarations
- Memory declarations
- Constant declarations
- Port declarations
- Flow blocks

### Planned

- Semantic analysis (`tyr-sema`)
- High-Level Intermediate Representation (`tyr-hir`)
- Backend framework (`tyr-backend`)
- Compiler driver (`tyr-driver`)
- Command-line interface (`tyrc`)
- Optimisation passes
- Code generation
- Hardware simulation support

### Current Compiler Pipeline

```text
          Source (.tyr)
                │
                ▼
      ┌──────────────────┐
      │   tyr-lexer      │
      └──────────────────┘
                │
             Tokens
                │
                ▼
      ┌──────────────────┐
      │   tyr-parser     │
      └──────────────────┘
                │
               AST
                │
                ▼
      ┌──────────────────┐
      │  tyr-sema        │   (Planned)
      └──────────────────┘
                │
                ▼
      ┌──────────────────┐
      │   tyr-hir        │   (Planned)
      └──────────────────┘
                │
                ▼
      ┌──────────────────┐
      │ tyr-backend      │   (Planned)
      └──────────────────┘
                │
                ▼
          Target Hardware
```

### Development Quality

Every commit to `main` is expected to pass:

```bash
cargo check
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

The project follows a test-first, warning-free development workflow. Pull requests are expected to satisfy all of the above checks before merging.
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