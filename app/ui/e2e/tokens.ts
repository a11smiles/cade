import { readFileSync } from 'node:fs';

interface ColorToken {
  name: string;
  value: string | Record<string, string>;
}

const tokens = JSON.parse(readFileSync(new URL('../../../design/system/tokens.json', import.meta.url), 'utf8')) as {
  color: { tokens: ColorToken[] };
};

/** A color token's value in `theme`, as the `rgb(r, g, b)` string computed styles report. */
export function tokenRgb(name: string, theme: 'dark' | 'light'): string {
  const token = tokens.color.tokens.find((t) => t.name === name);
  if (!token) throw new Error(`no color token ${name}`);
  const hex = typeof token.value === 'string' ? token.value : token.value[theme];
  if (!hex) throw new Error(`token ${name} has no ${theme} value`);
  const n = parseInt(hex.slice(1, 7), 16);
  return `rgb(${(n >> 16) & 255}, ${(n >> 8) & 255}, ${n & 255})`;
}
