# Security Policy

## Supported Versions

We actively provide security updates for the following versions of `zero-copy-env`:

| Version | Supported |
|--------|-----------|
| latest  | Yes       |
| < latest | No        |

Users are strongly encouraged to always use the latest stable release.

---

## Security Model

`zero-copy-env` is designed with performance and minimal overhead in mind. It operates under the following assumptions:

- Environment variables are provided by the operating system at process startup
- A snapshot of the environment is taken at initialization time
- No runtime mutation of environment variables is expected

---

## Important Security Considerations

### 1. Snapshot assumptions

This crate assumes that the process environment memory (`environ` on Unix systems) remains valid for the lifetime of the process.

If external unsafe code modifies environment memory, behavior is undefined.

---

### 2. Unsafe FFI usage

This crate uses unsafe FFI to access OS-level environment memory.

The unsafe blocks are:
- Required for interacting with system-level APIs
- Isolated and documented
- Not exposed to safe public APIs

---

### 3. Memory safety

The crate does not allocate or free environment memory. However:

- It depends on the OS-managed memory layout
- It assumes stable pointers provided by the operating system

---

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly.

Do NOT open a public issue for security vulnerabilities.

Instead, report it via:

- GitHub Security Advisories (preferred)
- Or private contact with maintainers

We will acknowledge receipt within a reasonable timeframe and work on a fix.

---

## Disclosure Policy

When a vulnerability is confirmed:

1. A fix will be developed privately
2. A new patched version will be released
3. A public advisory will be published after release

We follow responsible disclosure practices.

---

## Scope

This security policy applies to:

- Core crate functionality
- FFI interaction with OS environment
- Snapshot initialization logic

It does not cover:
- User-provided unsafe code
- External dependencies outside this crate

---

## Acknowledgement

Security issues are treated with priority and seriousness. We appreciate responsible disclosure from the community.