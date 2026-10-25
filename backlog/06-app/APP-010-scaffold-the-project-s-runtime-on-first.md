---
id: APP-010
title: Scaffold the project's runtime on first codegen
tags: [app, rust, feature]
epic: APP
priority: P0
depends_on: [APP-005]
---

## Description

On first codegen, scaffold a minimal Vite + React + TypeScript app with the design system package, ESLint, Prettier, Vitest and Testing Library, using only org-approved packages. Package install runs through the policy-gated command runner.

## Acceptance criteria

- [ ] Scaffold produces a buildable app with `npm run build` and `npm test`
- [ ] Only packages on the org allowlist are installed; others are blocked with a reason
- [ ] Lockfile is committed to the project repo
