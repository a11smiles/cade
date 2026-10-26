---
applyTo: "app/ui/**/*.css,app/ui/**/*.tsx,design/system/tokens.json"
---

- Use the CSS variables generated from `design/system/tokens.json` (`--bg-panel`, `--accent`, `--space-3`, `--radius-md` …). Literal colors, fonts, radii, shadows and spacing appear only in `tokens.json`.
- Edit `design/system/tokens.json`, never `app/ui/src/theme/tokens.css`; `pnpm --dir app/ui tokens` regenerates it. Every token has a dark and a light value.
- Use color roles as `design/system/README.md` defines them: `accent` for primary actions only, `accent-selection` for selection and focus, `accent-text`/`-chip`/`-soft` for Copilot, amber `policy*` for organization rules only, `tag-*` for output types only.
- Panels never cast shadows; only `shadow-float` (properties strip) and `shadow-page` (canvas pages) do.
- Desktop layout: the window is at least 1100 × 700 (`tauri.conf.json`) and opens at 1440 × 900. Keep the workspace layout from the design system: `topbar`, `rail`, 236–300px left panel, `copilot-column` on the right, floating properties strip.
- Hit areas are at least `control-md`. Focus is always visible (2px `accent-selection`, 2px offset), `prefers-reduced-motion` is respected, forced-colors mode stays usable, and color never shows state alone.
- Report tokens that fail contrast (4.5:1 text, 3:1 icons and large text) in either theme; don't change them without the designer.
