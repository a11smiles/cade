/// <reference types="node" />
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { expect, it } from 'vitest';

it('Fonts_OfflineEntry_UsesOnlyBundledFontAssets', () => {
  const entry = readFileSync(resolve('index.html'), 'utf8');
  expect(entry).not.toMatch(/<link\b[^>]*\bhref=["'](?:https?:)?\/\//i);

  const main = readFileSync(resolve('src/main.tsx'), 'utf8');
  expect(main).toContain("import './theme/fonts.css';");

  const css = readFileSync(resolve('src/theme/fonts.css'), 'utf8');
  expect(css).not.toMatch(/(?:https?:|\/\/|local\s*\()/i);
  const assets = [
    ['geist-latin.woff2', '1b5ebfb3a01a97343ac96873e6d59a8cb285c66012b6a1ac509cb2765e995ba8'],
    ['geist-latin-ext.woff2', 'f7604a53a00250f66db4b47dd44327ce2a43f6edec850d5217bb80238819efdd'],
    ['geist-mono-latin.woff2', 'b7ac144b394cbd81052d6397ec0c33397977b1d7e9bc095e744e652a378c6fb3'],
    ['geist-mono-latin-ext.woff2', '7d5609538a79545ec945e019771ed562e2c65a6c1c3c4ccc4ea34372e277c396'],
  ];
  for (const [name, hash] of assets) {
    expect(css).toContain(`url('../assets/fonts/${name}')`);
    const bytes = readFileSync(resolve('src/assets/fonts', name));
    expect(createHash('sha256').update(bytes).digest('hex')).toBe(hash);
  }
});
