# Rust Migration Roadmap

## Current Cut

The repository root now contains the Rust product foundation. The prior PowerShell/Electron implementation is archived at `-old-parts-bin-temp`.

All migration work must follow the production readiness principle: real production paths first, with simulations isolated to Stress Test Cycle and debugging environments.

Implemented modules:

- `hardware`: host inspection and binary SHA-256 integrity reporting.
- `trim`: dry-run-first local process/service trim plans.
- `bridge`: resource deficit calculation for RAM, vRAM, vCPU, and cloud storage.
- `launcher`: URL or executable launch preparation.
- `plugins`: JSON plugin manifest discovery.
- `package`: marketplace manifest rendering.

## Next Engineering Milestones

0. Preserve the product/runtime split.
   - Headless CLI/library is the source of truth.
   - GUI is only a wrapper for the packaged commercial product.
   - AnarkI zkEVM integrations must call the headless engine, not the GUI.

1. Replace trim placeholders with a policy engine.
   - Maintain per-OS allowlists and blocklists.
   - Add reversible recovery actions for every destructive trim.
   - Require explicit consent for shell/session teardown actions.
   - Keep simulated trim results confined to stress/debug mode.

2. Implement real cloud bridge providers.
   - Define a provider trait for allocation, health check, release, and billing metadata.
   - Add providers for AnarchI mainnet, Akash, Kubernetes, and generic HTTP cluster APIs.
   - Encrypt and store API keys with OS keychain support.
   - Do not claim cloud bridging in packaged builds until at least one provider performs real allocation and release.

3. Add workload profiles.
   - AI inference profile.
   - Blockchain node profile.
   - Browser container profile.
   - Game/render profile.

4. Harden plugin execution.
   - Support signed WASM plugins or native sidecar plugins.
   - Verify manifests before load.
   - Sandbox plugin permissions by hook.

5. Ship installers.
   - Windows MSI or NSIS.
   - macOS app bundle and notarization.
   - Linux AppImage or deb/rpm.

6. Add telemetry-free licensing.
   - Offline license file.
   - Optional online activation.
   - Clear privacy posture for market trust.

7. Add daemon mode for dedicated maintenance agents.
   - Local authenticated API.
   - Health monitoring loop.
   - Policy-approved trim execution.
   - Provider allocation/release lifecycle.
   - Plugin audit and update lifecycle.
