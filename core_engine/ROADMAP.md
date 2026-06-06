# Roadmap

## Phase 1: Rust Core Migration

- Port production-safe behavior from the archived PowerShell implementation.
- Implement trim policy and recovery actions.
- Add config persistence.
- Add tests for inspect, trim planning, bridge planning, and plugin manifests.
- Keep simulation behavior confined to Stress Test Cycle and debug environments.

## Phase 2: Headless Runtime

- Add daemon mode for dedicated maintenance agents.
- Add local authenticated API.
- Add health loop and pressure monitoring.
- Add policy-approved trim execution.
- Add provider allocation and release lifecycle.

## Phase 3: Cloud Bridge Providers

- Define provider trait.
- Implement first real provider.
- Add credential storage.
- Add allocation health checks and release paths.
- Add cost/quota metadata where providers support it.

## Phase 4: Plugin Marketplace

- Enforce manifest schema.
- Add signatures.
- Add permission policy.
- Add install/update/remove flows.
- Add paid plugin metadata.

## Phase 5: Packaged Product

- Add Tauri GUI wrapper for standalone customer product.
- Add installer.
- Add final commercial legal docs.
- Add release signing.
- Add market listing assets.

