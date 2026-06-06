# Changelog

All notable changes to AnarchI Core will be documented in this file.

This project follows pre-release versioning until the first production release.

## [0.2.0] - 2026-06-01

### Added

- Rust-first project scaffold.
- Headless CLI and library architecture.
- Hardware inspection module.
- Trim planning module.
- Cloud bridge planning module.
- Plugin manifest loading.
- Marketplace manifest draft.
- GUI prototype for future packaged product.
- Production readiness principle.
- GitHub-facing legal and community documentation.
- Explicit runtime modes for production, dry-run, stress-test, and debug.
- Trim policy metadata with action IDs, risk, admin requirements, and recovery actions.
- Plugin manifest validation and plugin policy defaults.
- Brand notes and starter SVG marks.

### Changed

- Archived legacy PowerShell/Electron project under `-old-parts-bin-temp`.

### Known Gaps

- Real cloud provider allocation is not implemented yet.
- Trim actions need policy hardening and recovery validation.
- Plugin execution and signature validation are not production-complete.
- Rust tooling was not available in the current environment for `cargo check`.
