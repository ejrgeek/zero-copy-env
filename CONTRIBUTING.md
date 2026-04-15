# Contributing to zero-copy-env

Thank you for considering contributing to zero-copy-env.

This project focuses on high-performance, snapshot-based environment variable access with strict guarantees around runtime behavior and memory usage.

---

## Principles

Before contributing, please understand the core principles of this project:

### 1. Snapshot-based model
Environment variables are read once and cached at initialization time. This behavior must not be broken.

### 2. No hot-path allocations
The `get()` function must remain allocation-free after initialization.

### 3. Predictable performance
All runtime lookups must remain O(1) average time complexity.

### 4. Safety assumptions must be explicit
Unsafe behavior is allowed only when clearly documented and justified.

---

## Types of contributions

We welcome contributions in the following areas:

### Bug fixes
- Memory safety improvements
- Edge case handling in environment parsing
- Platform-specific fixes (Unix / Windows compatibility)

### Performance improvements
- Benchmark-driven optimizations
- Cache locality improvements
- Alternative internal data structures (e.g. Vec vs HashMap)

### Documentation
- Improving clarity of README and inline docs
- Adding usage examples
- Explaining safety model better

### Testing
- Expanding integration test coverage
- Adding platform-specific test cases
- Fuzz testing for environment parsing

---

## Code guidelines

Please follow these rules when submitting code:

### Formatting
- Use `rustfmt`
- Run `cargo fmt` before submitting

### Clarity
- Prefer explicit code over clever code
- Keep unsafe blocks small and documented

### Performance changes
Any change affecting performance must include:

- Benchmark results (before/after)
- Explanation of trade-offs
- Justification for the change

Use `criterion` benchmarks when applicable.

---

## Commit guidelines

We recommend clear, structured commit messages:

Examples:
- `fix: handle empty environment variables safely`
- `perf: improve lookup cache locality`
- `docs: clarify snapshot initialization model`

---

## Pull Request process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests:
   ```bash
   cargo test
    ```
5. Run benchmarks:
    ```bash
    cargo bench
    ```
6. Submit a pull request

## Unsafe code policy
Unsafe code is allowed only when:

- It is required for FFI interaction with the OS
- It is clearly documented
- It has been reviewed for correctness assumptions

Any unsafe block must include a comment explaining:

- Why it is needed
- Why it is safe under current assumptions


## Platform support

The crate currently targets:

- Unix-like systems (primary support)
- Windows (planned / partial support depending on backend)

Platform-specific contributions are welcome.

## Discussion

For major changes, consider opening a discussion first before implementing.

This includes:
- Changing the snapshot model
- Introducing new backend strategies
- Modifying performance characteristics

## Thank you
Your contributions help improve performance-critical infrastructure tooling in Rust ecosystems.