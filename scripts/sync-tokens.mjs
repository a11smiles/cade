// Generates app/ui/src/theme/tokens.css from design/system/tokens.json.
// Run by `pnpm dev` / `pnpm build` in app/ui. Never edit tokens.css by hand.
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const t = JSON.parse(readFileSync(resolve(root, 'design/system/tokens.json'), 'utf8'));
const themes = t.color.themes.map((x) => x.id);
const val = (v, theme) => (typeof v === 'string' ? v : v[theme] ?? v[themes[0]]);

const themed = (theme) =>
  [...t.color.tokens, ...(t.shadow?.tokens ?? [])].map((k) => `  --${k.name}: ${val(k.value, theme)};`).join('\n');
const plain = ['spacing', 'radius', 'size']
  .flatMap((f) => t[f]?.tokens ?? [])
  .map((k) => `  --${k.name}: ${k.value};`)
  .join('\n');
const fonts = Object.entries(t.type.families).map(([k, v]) => `  --font-${k}: ${v};`).join('\n');
const styles = t.type.groups
  .flatMap((g) => g.styles.map((s) => ({ ...s, family: s.family ?? g.family })))
  .map((s) => `.t-${s.name} { font-family: var(--font-${s.family}); font-size: ${s.fontSize}; line-height: ${s.lineHeight}; font-weight: ${s.fontWeight}${s.letterSpacing ? `; letter-spacing: ${s.letterSpacing}` : ''}; }`)
  .join('\n');

const [first, second] = themes;
const css = `/* Generated from design/system/tokens.json by scripts/sync-tokens.mjs. Do not edit. */
:root, [data-theme="${first}"] {
${themed(first)}
}
[data-theme="${second}"] {
${themed(second)}
}
@media (prefers-color-scheme: ${second}) {
  [data-theme="system"] {
${themed(second).replace(/^/gm, '  ')}
  }
}
:root {
${plain}
${fonts}
}
${styles}
`;
writeFileSync(resolve(root, 'app/ui/src/theme/tokens.css'), css);
console.log(`tokens.css: ${t.color.tokens.length} colors, themes ${themes.join(', ')}`);
