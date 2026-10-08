# OmniRuntime

### High-performance cross-language meta-compiler & VM environment (Rust)

Unified execution · LLVM-backed · Multi-language bridge experiments

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Experimental-yellow)](#)

> Experimental unified runtime aiming to build, run, and bridge multiple languages from a single engine.

---

## Goals

- Single binary / environment for multiple language workflows
- LLVM IR path for performance-critical code
- Cross-language call bridges (research stage)
- Simple CLI: `omni run`, `omni build`, `omni execute`

---

## Planned / experimental language support

| Language | Mode | Notes |
|----------|------|-------|
| Rust | Native | Core engine |
| C / C++ | LLVM JIT path | Experimental |
| Python | Embedded interpreter path | Experimental |
| JavaScript | V8/QuickJS style integration | Planned |
| NovaQL | Native parser | Related project |

---

## Build

```bash
git clone https://github.com/sayan9168/OmniRuntime.git
cd OmniRuntime
cargo build --release
```

---

## Example CLI (target surface)

```bash
omni run script.py
omni build main.cpp --optimize
omni execute --bridge rust_logic.rs script.py
```

Exact commands depend on the current binary entrypoint in `src/`.

---

## Project structure

```text
OmniRuntime/
├── src/
│   ├── core/       # VM / LLVM bridge
│   ├── parsers/    # Language frontends
│   └── runtime/    # Unified execution
├── tests/
├── scripts/
└── Cargo.toml
```

---

## Status

**Experimental research project.** APIs and language coverage are evolving. Not production-ready.

---

## Author

[Sayan Mahata](https://github.com/sayan9168)
