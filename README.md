# AnarchI Core

AnarchI Core is a Rust-first engine for reclaiming local machine resources and bridging remaining workload gaps through configured vRAM, vCPU, cloud storage, and cluster providers.

The core engine is intentionally headless. The CLI/library form is the integration target for AnarkI zkEVM and dedicated maintenance agents. The GUI under `gui/` is a packaging layer for the standalone commercial product once the main logic is finalized and tested.

Core rule: AnarchI Core is written from the gate to be production ready, with simulations confined to Stress Test Cycle and debugging environments. See `docs/production-readiness-principle.md`.

The old PowerShell/Electron implementation has been archived in `-old-parts-bin-temp`.

## Product Shape

- `inspect`: report host capacity and executable integrity.
- `trim`: stop nonessential background services and processes by profile.
- `bridge`: calculate deficits and prepare a cloud cluster allocation request.
- `run`: combine inspect, trim, bridge planning, plugin hooks, and target launch.
- `plugins`: list installed plugin manifests.
- `package`: emit market-sale metadata for release packaging.

By default, trimming is dry-run. Use `--apply` only when you want the engine to change the host system.

## Integration Modes

- Headless CLI: operator and agent-managed runtime for AnarkI zkEVM nodes.
- Rust library: native embedding point for sidecars, agents, and future service APIs.
- GUI wrapper: packaged product interface for human customers.

## Quick Start

```powershell
cargo run -- inspect
cargo run -- trim --profile standard
cargo run -- trim --profile standard --runtime-mode dry-run
cargo run -- bridge --ram-gb 16 --vram-gb 8 --api-key $env:ANARCHI_API_KEY
cargo run -- run --target https://example.com --ram-gb 8 --vram-gb 4
```

## Release Build

```powershell
cargo build --release
```

The resulting binary will be at `target\release\anarchi-core.exe` on Windows.

## Repository Policies

This repository is proprietary source-available unless a separate written agreement says otherwise.

- License: `LICENSE.md`
- Terms: `TERMS.md`
- Privacy: `PRIVACY.md`
- Security: `SECURITY.md`
- Support: `SUPPORT.md`
- Contributing: `CONTRIBUTING.md`
- Roadmap: `ROADMAP.md`
- Changelog: `CHANGELOG.md`

Legal documents are founder drafts for GitHub publication and should be reviewed by qualified counsel before commercial sale, marketplace distribution, or enterprise deployment.
