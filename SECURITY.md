# Security

Please report vulnerabilities privately through GitHub's **Report a vulnerability** (Security › Advisories) on this repository. Don't open a public issue.

Include the CADE version, your OS, and steps to reproduce. You'll get an acknowledgement within 5 business days.

## Scope notes

- CADE runs locally. Designs, prompts and Copilot threads stay on your machine and in your own repositories.
- Tokens are kept in the OS keychain. If you find a token, key or prompt written to disk or logs, that's in scope.
- Imported files (Visio, PowerPoint, Figma, Spark repos) are untrusted input. Parser crashes and macro execution are in scope.
