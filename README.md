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

## Compiler Roadmap

- ✅ v0.1 — Lexer
- ✅ v0.2 — Parser infrastructure
- ✅ v0.3 — Signal declarations
- ⏳ v0.3.x — Hardware declarations (constants, registers, memories, ports)
- ⏳ v0.4 — Semantic analysis
- ⏳ v0.5 — High-Level IR
- ⏳ v0.6 — Backend framework
- ⏳ v0.7 — First code generation target
- ⏳ v0.8 — Optimisation passes
- ⏳ v0.9 — Feature-complete beta
- ⏳ v1.0 — Stable Tyr compiler

---

## Current Status

**Current Release:** **v0.3.0**

The Tyr compiler (`tyrc`) is under active development. The project currently implements the complete frontend foundation, including lexical analysis, AST construction and recursive-descent parsing of modules and signal declarations.

### Implemented

| Component | Status | Notes |
|-----------|:------:|-------|
| Workspace | ✅ | Modular Cargo workspace architecture |
| CI/CD | ✅ | GitHub Actions (check, fmt, clippy, tests) |
| `tyr-common` | ✅ | Diagnostics, spans, source management and common utilities |
| `tyr-lexer` | ✅ | UTF-8 lexer with keywords, identifiers, literals, operators, punctuation, directives and diagnostics |
| `tyr-ast` | ✅ | AST nodes for compilation units, modules, identifiers, types and signal declarations |
| `tyr-parser` | 🟡 | Recursive-descent parser supporting compilation units, modules and signal declarations |
| Unit Tests | ✅ | Comprehensive unit tests for implemented frontend components |

### Language Features

Currently supported syntax:

```tyr
module Main

signal clk  : bit;
signal data : trit;

end
```

Supported primitive types:

- `bit`
- `trit`
- `clock`
- `event`

Supported integer literal encodings:

- `0b` — Binary
- `0o` — Octal
- `0x` — Hexadecimal
- `0t` — Traditional unbalanced ternary (`0`, `1`, `2`)
- `0tf` — Fractional unbalanced ternary (`0`, `h`, `1`)
- `0tb` — Balanced ternary (`n`, `0`, `1`)

### In Progress

- Constant declarations
- Register declarations
- Memory declarations
- Port declarations (`input`, `output`, `inout`)
- Flow blocks
- Composite type parsing (`bus`, `array`)

### Planned

- Semantic analysis (`tyr-sema`)
- Symbol table construction
- Type checking
- High-Level Intermediate Representation (`tyr-hir`)
- Backend framework (`tyr-backend`)
- Compiler driver (`tyr-driver`)
- Command-line interface (`tyrc`)
- Optimisation passes
- Hardware code generation
- Hardware simulation support

### Current Compiler Pipeline

```text
                Source (.tyr)
                      │
                      ▼
        ┌────────────────────────┐
        │       tyr-lexer        │
        └────────────────────────┘
                      │
                   Tokens
                      │
                      ▼
        ┌────────────────────────┐
        │      tyr-parser        │
        └────────────────────────┘
                      │
                      ▼
        ┌────────────────────────┐
        │        tyr-ast         │
        └────────────────────────┘
                      │
                      ▼
        ┌────────────────────────┐
        │       tyr-sema         │   (Planned)
        └────────────────────────┘
                      │
                      ▼
        ┌────────────────────────┐
        │        tyr-hir         │   (Planned)
        └────────────────────────┘
                      │
                      ▼
        ┌────────────────────────┐
        │     tyr-backend        │   (Planned)
        └────────────────────────┘
                      │
                      ▼
               Target Hardware
```

### Development Quality

Every commit to `main` is expected to successfully complete the **Tyr Dance**:

```bash
cargo check
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

The project follows a warning-free, test-driven development workflow. Every change is validated through the complete frontend before being committed.

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