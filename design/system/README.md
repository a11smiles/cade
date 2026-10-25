# CADE

CADE (Copilot ADE) is a local, cross-platform design workbench driven by GitHub Copilot. This system is the app's own interface: the chrome around the canvas, the Copilot column, panels, prompts and states. It is not the design system of what users make. Their own system (for example "Contoso DS") renders inside the canvas with its own colors.

Dark is the primary theme. Light is a full peer, and every token has both values.

## Content fundamentals

CADE talks like a capable colleague who respects the user's time.

- **Say what happened, then what still works.** "Copilot is paused until Nov 1. Your prompt is saved. You can keep editing by hand, export and ship."
- **Short and concrete.** Name the real thing: "Apply 6 changes", "Open pull request", "kv-claims + private endpoint". Avoid "Proceed", "OK", "Something went wrong".
- **Sentence case** everywhere. Uppercase only for section labels and type tags.
- **Copilot speaks in results.** It starts with what it did ("Built Step 1 for mobile and desktop from 9 Contoso components"), then what's next or what it needs. No greetings, no apologies, no "I hope this helps".
- **Questions come with answers.** When Copilot needs a decision, it offers the likely answers as buttons.
- **Governance is named, not hidden.** When a rule changes what happens, say which rule ("Rule: tools.network = off").
- **Never imply lost work.** Every failure message says where the user's prompt or changes went.
- **Placeholders, not invented numbers.** Unknown values show as `[X]` until real data is attached.
- Units: tokens (not requests) for Copilot usage; "4.2M / 10M".

## Visual foundations

**Color roles.** Graphite surfaces (`bg-app` → `bg-chrome` → `bg-panel` → `bg-raised`) separate regions by lightness, with `border` hairlines between them. One violet does the work:

- `accent` is for primary actions only.
- `accent-selection` is for selection outlines, handles and focus rings.
- `accent-text`, `accent-chip` and `accent-soft` mark anything Copilot says, sees or staged.

**Amber means governance and nothing else.** `policy`, `policy-text` and `policy-solid` appear only for organization rules, permission prompts, warnings and comment pins.

Status uses `success`, `danger` and `policy`. Tones are always told apart by icon and words as well as color.

**Type tags.** Each output type owns one pair: `tag-app`, `tag-image`, `tag-deck`, `tag-document`, `tag-motion`, `tag-diagram`, `tag-architecture` and `tag-data`, each with its `-text` partner. Never use them for anything else.

**Type.** Geist for the interface and Geist Mono for anything literal: token names, paths, sizes, SHAs, code. The interface sizes are `body` 13/20, `small` 12/18, `caption` 11/16 and `title` 14/20. Larger titles are for pages, not panels.

**Space and shape.**
- Spacing uses a 4px base (`space-1` … `space-8`).
- Corners grow with the size of the surface: `radius-xs` for op badges, `radius-md` for buttons and inputs, `radius-xl` for callouts and the composer, `radius-2xl` for the floating properties strip and Start cards.
- Panels never cast shadows. Only the floating strip (`shadow-float`) and pages and slides on the canvas (`shadow-page`) do.

**Canvas.** `bg-canvas` with a `canvas-dot` grid at 20px. User content sits on it in its own colors and is never tinted by CADE's theme.

**Workspace layout.**
- Top bar is `topbar` (52px) tall.
- Optional tool rail is `rail` (52px) wide.
- Left panel is 236–300px wide.
- The canvas fills the middle.
- The Copilot column is `copilot-column` (352–380px) wide on the right.
- The contextual `PropertiesStrip` floats `space-5` above the canvas bottom. It replaces a fixed inspector, so the right side stays Copilot's.

**Accessibility.**
- Every text token pair here passes 4.5:1 in both themes.
- Focus rings are 2px `accent-selection` with a 2px offset.
- Hit areas are at least `control-md`, and `touch-min` for phone preview.

## Iconography

- Outline icons on a 24px grid with a 1.8px stroke, round caps and joins, drawn in `currentColor`. The set ships as the `Icon` component.
- **Sparkle** always means Copilot and **shield** always means an organization rule. Neither is used for anything else.
- No emoji, no filled glyph icons except play, and no third-party brand marks in CADE's own UI.
- Azure architecture diagrams use Microsoft's official Azure Architecture Icons, loaded from the user's `assets/icons/azure/` folder. They are content, not CADE iconography.

## Logo

The CADE mark is a violet rounded square (`accent`) holding a white brush stroke, with an amber (`policy-solid`) dot at its tip: Copilot does the drawing, inside the lines the organization sets. See the Logos group. Use the mark at 20–36px in the top bar and the full lockup with the "CADE" wordmark in Geist 700 on first run.

## Components

`window.CADE` exposes Button, IconButton, Icon, SegmentedControl, TypeTag, StatusPill, Callout, TextField, ContextChip, ChangeList, PropertiesStrip, Message and Composer. Each card has its own guidelines. In the app they are reimplemented in `app/ui` against the same tokens, and this bundle is the reference.
