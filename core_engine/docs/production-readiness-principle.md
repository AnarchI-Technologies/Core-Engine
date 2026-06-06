# Production Readiness Principle

## Never-Changing Truth

AnarchI Core is written from the gate to be production ready, with simulations confined to Stress Test Cycle and debugging environments.

## Engineering Rules

- Production paths must represent real behavior, real provider contracts, real safety checks, and real recovery guarantees.
- Simulated cloud allocation, simulated trim success, fake resource claims, mock billing, and placeholder provider responses must be isolated behind explicit stress/debug flags.
- Defaults must be safe, inspectable, and reversible.
- Dry-run planning is allowed in production because it is an operational safety feature, not a simulation.
- Any code path that does not actually mutate resources, allocate provider capacity, or launch the real workload must identify itself as stress/debug behavior.
- Packaging must never ship with stress/debug behavior enabled by default.

## Accepted Runtime Modes

- `production`: real inspection, real policy checks, real trims when approved, real provider allocations.
- `dry-run`: production-safe planning mode that reports what would happen without mutation.
- `stress-test`: contained simulation mode for load, failure, provider, and plugin behavior testing.
- `debug`: developer-only diagnostics and local troubleshooting.

## Release Gate

A packaged release is not valid unless:

- Stress/debug code paths are disabled by default.
- Provider integrations either execute real allocation/release calls or are excluded from production claims.
- Trim actions have matching recovery behavior where possible.
- Plugin loading validates manifests and permission policy before execution.
- Credentials are never logged, embedded, or exposed to plugins without explicit permission.
