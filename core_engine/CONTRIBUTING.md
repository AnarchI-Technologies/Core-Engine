# Contributing

Thank you for your interest in AnarchI Core.

## Project Principle

AnarchI Core is written from the gate to be production ready, with simulations confined to Stress Test Cycle and debugging environments.

Contributions must preserve that rule.

## Before Opening A Pull Request

- Keep production behavior real, safe, inspectable, and recoverable.
- Put simulated behavior behind explicit stress/debug configuration.
- Do not log credentials or sensitive workload data.
- Keep the headless core independent from the GUI wrapper.
- Add or update documentation when behavior changes.
- Prefer small, focused pull requests.

## Development Areas

- Rust headless core.
- Trim policy and recovery actions.
- Cloud bridge provider contracts.
- Plugin manifest validation and signing.
- Marketplace packaging.
- GUI wrapper for the standalone product.
- Documentation and release checklists.

## Contribution License

By contributing, you agree to the contribution terms in `LICENSE.md`.

## Security Issues

Do not submit public pull requests or issues for vulnerabilities until coordinated disclosure is complete. See `SECURITY.md`.

