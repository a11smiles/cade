---
id: FOUND-002
title: Scaffold app/ui with Vite, React and strict TypeScript
tags: [found, chore, ts, ui]
epic: FOUND
priority: P0
depends_on: []
---

## Description

Create the webview front end in `app/ui` using pnpm, Vite, React 18 and TypeScript in strict mode. Add ESLint (typescript-eslint, react-hooks, jsx-a11y), Prettier, Vitest and a path alias layout (`@/components`, `@/surfaces`, `@/ipc`). This is the chrome for every workspace; the canvas itself is native and is not rendered here.

## Acceptance criteria

- [ ] `pnpm -C app/ui build` produces a static bundle Tauri can load
- [ ] `tsc --noEmit` passes with `strict`, `noUncheckedIndexedAccess` and `exactOptionalPropertyTypes`
- [ ] ESLint includes jsx-a11y and fails CI on violations
- [ ] Vitest runs with jsdom and a sample component test
- [ ] Node and pnpm versions are pinned via `packageManager` and `.nvmrc`
