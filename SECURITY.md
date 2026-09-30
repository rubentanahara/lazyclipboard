# Security policy

## Reporting a vulnerability

Report it privately through GitHub: **Security → Report a vulnerability** on this repository. Do not open a public issue.

Include the affected version, your OS, and steps to reproduce. Expect an acknowledgement within 7 days.

## Scope

lazyclipboard stores items on your device and keeps AI provider keys in the OS keychain. Reports about these are in scope:

- reading or writing items or keys from another process or user
- HTML from a captured item running as code inside the app
- keys or item content leaving the device other than to the AI provider you configured
- a tampered update being accepted by the updater

## Release signing

Signing keys and notarization credentials live only in GitHub Actions secrets, used by the release workflow on `v*` tags. They are never committed to this repository.
