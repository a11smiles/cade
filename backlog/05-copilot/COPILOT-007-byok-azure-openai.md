---
id: COPILOT-007
title: Add the Azure OpenAI BYOK provider
tags: [copilot, feature, rust]
epic: COPILOT
priority: P1
depends_on: [COPILOT-006]
---

## Description

Support Azure OpenAI deployments: endpoint URL, deployment name, API version and either an API key (keychain) or Microsoft Entra ID sign-in via device code. Reuse the OpenAI wire format from COPILOT-006 with Azure-specific URL construction and auth headers.

## Acceptance criteria

- [ ] User can add an Azure OpenAI connection with endpoint, deployment and API version and run a prompt
- [ ] Entra ID device-code sign-in works and the token is refreshed silently before expiry
- [ ] Wrong deployment name returns a clear 'Deployment not found' error
- [ ] Connection details except secrets are stored in app config; secrets only in the keychain
