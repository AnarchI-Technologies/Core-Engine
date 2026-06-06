# Release Checklist

## Code

- `cargo check`
- `cargo test`
- Linting and formatting
- Dependency audit
- Secret scan
- Plugin schema validation

## Product

- Production readiness principle verified.
- Stress/debug behavior disabled by default.
- No simulated provider behavior in production claims.
- Trim actions documented with recovery paths.
- Cloud providers documented with allocation and release behavior.
- GUI wrapper points to the same headless engine.

## Legal

- License reviewed.
- Terms reviewed.
- Privacy policy reviewed.
- Security policy reviewed.
- Third-party notices generated.
- Marketplace terms finalized.

## Packaging

- Release binary built.
- Installer built.
- Binary signed.
- Checksums generated.
- Release notes written.
- Known limitations documented.

