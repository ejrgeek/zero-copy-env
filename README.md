[![CI](https://github.com/ejrgeek/zero-copy-env/actions/workflows/ci.yml/badge.svg)](https://github.com/ejrgeek/zero-copy-env/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/zero-copy-env.svg)](https://crates.io/crates/zero-copy-env)
[![Docs.rs](https://docs.rs/zero-copy-env/badge.svg)](https://docs.rs/zero-copy-env)
[![License](https://img.shields.io/crates/l/zero-copy-env.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/zero-copy-env.svg)](https://crates.io/crates/zero-copy-env)

# Zero-Copy-Env

Zero-copy environment variable access using OS-provided process memory.

## Overview

This crate reads environment variables directly from the process memory (`environ` on Unix systems), builds a snapshot at initialization time, and provides fast O(1) lookups during runtime.

## Model

The system works in three phases:

1. Process environment is provided by the OS
2. A snapshot is created at first access
3. All lookups are served from an in-memory map

After initialization:
- No syscalls
- No environment scanning
- No per-query allocation

## Characteristics

- Snapshot at initialization (lazy, thread-safe)
- O(1) average lookup (HashMap-based backend)
- No per-query allocation
- Designed for predictable runtime performance

## Safety

This crate assumes that environment memory remains stable for the lifetime of the process.

It may become invalid if:
- The process mutates environment variables at runtime (`std::env::set_var`, `remove_var`)
- External unsafe FFI modifies process environment memory

This behavior is not checked at runtime.

## Performance Model

- Initialization cost: O(n) (environment scan)
- Lookup cost: O(1) average
- Runtime cost after init: constant memory access only

## Usage

```rust
use zero_copy_env::get;

fn main() {
    let path = get("PATH");
    println!("{:?}", path);
}
```

## Testing

Run tests:

```bash
cargo test
```

Run Benchmarks:

```bash
cargo bench
```

## Contribution

Contributions are welcome.

Please follow these guidelines:

**1. Keep the zero-copy model intact**

Any contribution must respect the core design principles:

 - No unnecessary allocations in hot paths
 - No per-call syscalls
 - Preserve snapshot-based architecture

**2. Keep the API stable**

Avoid breaking changes unless strictly necessary. If needed:

- Prefer additive changes
- Use feature flags for experimental behavior


**3. Performance matters**

If you propose changes that affect performance:

- Include benchmarks (`criterion`)
- Provide before/after comparisons when possible

**4. Code style**

- Follow standard Rust formatting (`cargo fmt`)
- Prefer explicit unsafe blocks with justification
- Keep modules minimal and focused

## Roadmap

Planned improvements:

- Optional Vec-based backend for cache locality optimization
- Optional no-std support refinement
- Extended benchmark suite across environments
- Optional hot-reload mode (feature-gated)

## License MIT
