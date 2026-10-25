---
id: APP-013
title: Generate tests from validation rules
tags: [app, rust, feature]
epic: APP
priority: P1
depends_on: [APP-012, COPILOT-019]
design: design/exports/v0001-2026-10-25/Code.dc.html
---

## Description

Provide a test-generation hook that writes Vitest + Testing Library tests for each validation rule and interaction. Copilot uses it when asked to 'write tests for this step', and results show in the Copilot panel.

## Acceptance criteria

- [ ] Tests are generated for each rule (valid, invalid, boundary)
- [ ] Generated tests run in the sandbox and report pass/fail in the panel
- [ ] Test files are named `<Component>.test.tsx` next to the component
