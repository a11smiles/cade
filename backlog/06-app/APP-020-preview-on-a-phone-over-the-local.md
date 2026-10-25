---
id: APP-020
title: Preview on a phone over the local network
tags: [app, rust, feature, security]
epic: APP
priority: P1
depends_on: [APP-019, APP-009]
design: design/exports/v0001-2026-10-25/Prototype.dc.html
---

## Description

Serve the prototype from this machine on the LAN with a one-time token and show a QR code. Phones on the same Wi-Fi open it in their browser. No cloud relay. Policy can disable it.

## Acceptance criteria

- [ ] Scanning the QR code opens the prototype on a phone on the same network
- [ ] Each session uses a random token and expires when the window closes
- [ ] The server binds only to the LAN interface and refuses requests without the token
- [ ] Policy setting `preview.lan: off` hides the feature
