# GUI Product Spec

## Role

The GUI is a commercial packaging layer for the standalone AnarchI Core product. It is not required for the AnarkI zkEVM build, where the same core logic should run headlessly through the CLI/library API and be maintained by dedicated agents.

For product customers, the GUI is the control room for nontechnical users and teams running dense AI-agent or blockchain workloads. It should show what the engine will trim, what resources are still missing, and where those deficits will be bridged before anything mutates the machine.

## Primary Views

- Dashboard: CPU, RAM, estimated vRAM, active profile, cluster bridge status, plugin health.
- Workload Launcher: target URL/executable, workload type, required RAM/vRAM/vCPU/storage, API key/provider selector.
- Trim Plan: dry-run actions, risk level, reversible recovery action, explicit apply button.
- Cloud Bridge: provider, endpoint, requested burst resources, allocation status, cost estimate placeholder.
- Plugin Marketplace: installed plugins, signed status, hook permissions, install/update/remove workflow.
- Logs: engine events, provider allocation events, plugin events, recovery events.

## GUI Implementation Direction

Recommended path: Tauri shell with this Rust crate as the command/library backend.

Why:

- Keeps the core engine in Rust.
- Produces small commercial desktop packages compared with Electron.
- Allows a polished web UI without turning the optimizer into JavaScript logic.
- Can expose the same engine API to the future AnarkI zkEVM integration layer.
- Keeps the packaged UI outside the critical headless runtime used by AnarkI zkEVM.

Alternative path: pure Rust `egui`/`eframe` if a fully native stack is preferred over web UI tooling.

## UX Principles

- Default to dry-run and explain impact before changes.
- Make cloud bridging visible as a resource allocation, not a magic claim.
- Treat plugins as products: show publisher, signature, permissions, price/license state, and hooks.
- Keep advanced controls available, but make the first-run path one screen: target, requirements, provider, run.
