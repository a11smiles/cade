---
id: APP-024
title: Ship an App project to a repo
tags: [app, rust, ts, feature]
epic: APP
priority: P0
depends_on: [GH-010, APP-011]
design: design/exports/v0001-2026-10-25/Ship.dc.html
---

## Description

The Ship screen picks repository and branch, output type (React + DS or design spec only), lists files, runs pre-flight (DS only, approved packages, secret scanning, accessibility, note that code scanning runs on the PR), and opens a PR with a Copilot-written description and CODEOWNERS reviewers.

## Acceptance criteria

- [ ] Repo and branch pickers list repos the user can push to
- [ ] Pre-flight runs locally and blocks shipping on any failure
- [ ] The PR includes generated code and the design source file
- [ ] Reviewers are suggested from CODEOWNERS
- [ ] 'Save as draft PR' opens a draft
