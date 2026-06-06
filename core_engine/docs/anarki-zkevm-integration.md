# AnarkI zkEVM Integration Notes

## Strategic Fit

AnarchI Core should become the optimization layer beneath AnarkI zkEVM-hosted AI-agent and human collaboration workflows. Its job is to keep physical systems from stalling while agent clusters socialize, plan, coordinate, build, and run business workflows.

## Runtime Shape

The AnarkI zkEVM build should use AnarchI Core as a headless subsystem. It should not depend on the packaged product GUI.

Expected forms:

- CLI process supervised by dedicated maintenance agents.
- Native Rust library embedded into a node sidecar.
- Future local HTTP/gRPC service for agent cluster orchestration.

The GUI remains useful for the standalone commercial tool, but it should stay outside the zkEVM runtime dependency path.

## Engine Boundary

The core engine should remain independent from the zkEVM layer:

- Input: workload target, requirements, profile, provider credentials, plugin policy.
- Output: local trim plan, resource bridge plan, launch plan, plugin lifecycle events, machine health status.
- No direct dependency on chain state in the base engine.

The zkEVM layer can then consume the engine as:

- A local node sidecar.
- A cluster agent runtime health manager.
- A marketplace-verifiable optimization service.
- A plugin host for monetized AI, business, and infrastructure tools.

## Modular API Direction

Expose the engine through multiple front doors:

- Rust library API for native integrations.
- CLI for operators and automation.
- GUI command API through Tauri for the packaged product only.
- Future local HTTP/gRPC sidecar API for agent clusters.

## Agent Maintenance Model

Dedicated agents should be able to:

- Monitor host pressure and request fresh trim/bridge plans.
- Apply approved trim profiles under policy.
- Rotate or validate provider credentials without exposing secrets to plugins.
- Install, update, disable, and audit plugins.
- Report machine health and resource allocation status back to the zkEVM orchestration layer.

## Plugin Economy

Plugins should be monetizable units with:

- Signed manifests.
- Declared hooks and permissions.
- Optional WASM modules or sidecar binaries.
- Versioned pricing/license metadata.
- Marketplace verification before install.

This lets the same optimization layer support new vertical tools without bloating the core binary.
