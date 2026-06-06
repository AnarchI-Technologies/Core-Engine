# GitHub Publication Checklist

Complete this before making the repository public or sharing it with partners.

## Identity

- Replace all `anarchi.local` contact placeholders with real monitored addresses.
- Confirm final spelling and branding for AnarchI, AnarkI zkEVM, and AnarchI Core.
- Add repository description, topics, and website URL in GitHub settings.

## Legal

- Attorney review of `LICENSE.md`.
- Attorney review of `TERMS.md`.
- Attorney review of `PRIVACY.md`.
- Confirm whether a contributor license agreement is needed.
- Generate third-party notices after Rust dependencies are finalized.

## Security

- Confirm private vulnerability intake email.
- Enable GitHub secret scanning.
- Enable branch protection on `main`.
- Require pull request review before merge.
- Decide whether public issues are enabled immediately.

## Product Claims

- Remove or qualify claims for features that are not production-complete.
- Keep cloud bridge claims limited to implemented providers.
- Keep stress/debug behavior clearly separated from production behavior.

## Repository Hygiene

- Confirm `-old-parts-bin-temp` should be committed or moved outside the public repo.
- Remove build artifacts, packaged executables, and dependency folders before initial push.
- Run formatting, tests, and dependency audit once Rust tooling is installed.
