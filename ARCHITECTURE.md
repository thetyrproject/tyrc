# tyrc Architecture

---

# Overview

The Tyr compiler is designed as a collection of independent Rust libraries.

Every compiler stage is implemented as an isolated crate with a clearly defined responsibility.

The compiler executable (`tyrc`) is intentionally small and acts only as the command-line frontend.

This separation encourages modularity, testability, and reuse by future tools such as language servers, formatters, simulators, and documentation generators.

---

# Design Principles

The compiler follows several architectural principles.

- Single responsibility
- Modular crates
- Specification-first development
- Technology-independent compilation
- Deterministic behaviour
- Test-driven evolution

---

# Compiler Pipeline

```
             Tyr Source
                  │
                  ▼
      Compiler Directive Processing
                  │
                  ▼
               Lexer
                  │
                  ▼
               Tokens
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

# Workspace

```
crates/

    tyr-common

    tyr-lexer

    tyr-parser

    tyr-ast

    tyr-sema

    tyr-hir

    tyr-backend

    tyr-driver

    tyrc
```

---

# Crate Responsibilities

## tyr-common

Shared compiler infrastructure.

Responsibilities:

- source management
- spans
- diagnostics
- file identifiers
- common utilities

No compiler stage shall introduce dependencies into this crate.

---

## tyr-lexer

Lexical analysis.

Responsibilities:

- tokenization
- keyword recognition
- literal recognition
- source traversal

Output:

```
Token Stream
```

---

## tyr-parser

Syntax analysis.

Responsibilities:

- grammar parsing
- syntax diagnostics
- AST construction

Output:

```
Abstract Syntax Tree
```

---

## tyr-ast

Defines the Abstract Syntax Tree.

Contains no parsing logic.

Contains no semantic analysis.

Only data structures.

---

## tyr-sema

Semantic analysis.

Responsibilities:

- name resolution
- scope management
- type checking
- semantic validation

Output:

```
Validated AST
```

---

## tyr-hir

Hardware Intermediate Representation.

Responsibilities:

- lowering
- elaboration
- canonical hardware representation

HIR exists independently of any backend.

---

## tyr-backend

Backend-independent code generation framework.

Future targets include:

- Verilog
- VHDL
- Simulation

---

## tyr-driver

Coordinates the complete compilation pipeline.

Responsible for:

- loading files
- invoking compiler stages
- reporting diagnostics

---

## tyrc

Command-line interface.

Responsibilities:

- parse command-line arguments
- configure compilation
- invoke tyr-driver

The executable intentionally contains minimal compiler logic.

---

# Dependency Graph

```
             tyr-common
                  │
        ┌─────────┼─────────┐
        │         │         │
        ▼         ▼         ▼
   tyr-lexer  tyr-ast  tyr-backend
        │         │
        └────┐    │
             ▼    ▼
          tyr-parser
               │
               ▼
           tyr-sema
               │
               ▼
            tyr-hir
               │
               ▼
          tyr-driver
               │
               ▼
              tyrc
```

Dependencies shall always flow downward.

Circular dependencies are prohibited.

---

# Development Philosophy

The compiler implements the Tyr Language Specification.

The specification defines the language.

The compiler does not define the language.

Whenever implementation behaviour differs from the specification, the specification shall be considered authoritative.

---

# Testing

Every crate shall contain unit tests.

The complete workspace shall pass:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

before changes are merged.

---

# Future Work

The architecture has been intentionally designed to support future tooling, including:

- Tyr Language Server (`tyrls`)
- Tyr Formatter (`tyrfmt`)
- Tyr Linter (`tyrlint`)
- Native Simulator (`tyrsim`)
- Additional compiler backends

These tools should reuse the compiler frontend whenever practical.

---

**The Tyr Project**

**One Language. Every Digital Architecture.**