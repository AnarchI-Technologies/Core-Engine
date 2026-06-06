# Security Policy

## Supported Versions

AnarchI Core is pre-release. Security review currently applies to the latest `main` branch and release candidates explicitly marked for testing.

## Reporting A Vulnerability

Please do not open a public GitHub issue for security vulnerabilities.

Send reports to: security@anarchi.local

Include:

- Affected version or commit.
- Operating system and environment.
- Reproduction steps.
- Expected and actual behavior.
- Impact assessment.
- Any logs, proof-of-concept details, or patches that help verification.

## Response Targets

- Initial acknowledgement: 5 business days.
- Triage target: 10 business days.
- Remediation timing depends on severity, exploitability, and release status.

## Security Boundaries

High-priority areas:

- Credential handling.
- Plugin manifest validation.
- Plugin signing and marketplace verification.
- Process/service trim policy.
- Recovery actions.
- Cloud bridge provider allocation and release.
- Logs and secret redaction.

## Safe Harbor

Good-faith security research is welcome when it avoids privacy violations, data destruction, persistence, lateral movement, social engineering, denial of service, or access to systems you do not own or have permission to test.

