# Market Readiness Checklist

## Product Promise

AnarchI Core is positioned as a plug-and-play optimization and resource bridge tool. The user enters a workload target and requirements; the engine trims unnecessary local overhead, computes remaining deficits, and bridges those deficits through configured vRAM, vCPU, cloud storage, and cluster APIs.

## Required Before Sale

- Replace simulated cloud bridge planning with at least one real provider integration.
- Add undo/recovery guarantees for every local trim action.
- Add admin privilege detection and clear prompts before privileged changes.
- Add signed release builds.
- Add final commercial license, privacy policy, refund policy, and support policy.
- Add automated tests for config parsing, bridge deficit math, plugin manifest validation, and trim planning.
- Run security review on process termination, credential handling, and plugin loading.

## Packaging Target

Primary launch SKU:

- Windows x86_64 executable.
- Installer with Start Menu entry.
- Bundled default config and sample plugin manifest.
- Documentation for provider API keys and supported workload profiles.

The GUI package should be treated as a customer-facing distribution of the same headless engine, not as a separate implementation.

Secondary SKUs:

- Linux x86_64 server/desktop build.
- macOS Apple Silicon build after notarization work.

## Differentiators To Build Toward

- One command to reclaim local resources and burst to cloud capacity.
- AI workload and blockchain node profiles.
- Transparent dry-run output before machine changes.
- Plugin ecosystem with signed marketplace packages.
- Privacy-first and telemetry-free by default.
