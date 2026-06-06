# Headless Runtime

## Purpose

The headless runtime is the production integration target for AnarkI zkEVM. It runs without a GUI and exposes the optimization layer to dedicated maintenance agents, node supervisors, and future sidecar APIs.

## Contract

Input:

- Workload target.
- RAM, vRAM, vCPU, and cloud storage requirements.
- Trim profile.
- Provider policy and credential availability.
- Plugin policy.

Output:

- Hardware pressure report.
- Local trim plan.
- Cloud bridge plan.
- Launch or supervision plan.
- Plugin lifecycle summary.
- Machine health events.

## Operating Modes

- `inspect`: passive host report.
- `trim`: dry-run or policy-approved local trimming.
- `bridge`: provider allocation planning.
- `run`: one-shot inspect/trim/bridge/launch flow.
- Future `daemon`: long-running agent-supervised service.

## Non-Goals

- No GUI dependency.
- No marketplace browsing in the critical node runtime.
- No direct blockchain dependency inside the optimization engine.
- No plugin execution without manifest validation and permission policy.

## Future Agent API

The next major step is a daemon mode with a local authenticated API:

- `POST /plan`
- `POST /trim/apply`
- `POST /bridge/allocate`
- `POST /bridge/release`
- `GET /health`
- `GET /plugins`

This keeps agent integration stable while preserving the CLI for operators.
