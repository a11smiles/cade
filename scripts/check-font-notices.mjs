// Run after Vite, from any working directory. No prebuilt dist is needed by units.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const requiredFile = (path) => {
  assert.ok(existsSync(resolve(root, path)), `Required font artifact or notice missing: ${path}`);
  return readFileSync(resolve(root, path));
};

test('Fonts_ProductionArtifact_PreservesCompleteNoticesAndFontBytes', () => {
  const license = requiredFile('app/ui/dist/fonts/OFL.txt');
  const provenance = requiredFile('app/ui/dist/fonts/PROVENANCE.md');
  assert.deepStrictEqual(license, requiredFile('app/ui/public/fonts/OFL.txt'),
    'Built OFL must equal the complete canonical notice');
  // Pinned Geist 1.4.01 OFL: terminal LF added, line 21 trailing space removed.
  assert.strictEqual(sha256(license),
    'f12f59163d92084b0473315ae4566cde0c1233d940ae464f5225a69c347cc691',
    'OFL must retain the complete approved copyright and license');
  assert.deepStrictEqual(provenance, requiredFile('app/ui/public/fonts/PROVENANCE.md'),
    'Built provenance must equal the complete canonical source record');

  const notice = requiredFile('NOTICE').toString('utf8');
  assert.ok(notice.includes('Geist Mono'), 'NOTICE must attribute both Geist families');
  assert.ok(notice.includes('Copyright 2024 The Geist Project Authors'),
    'NOTICE must include the matching font copyright');
  assert.match(notice, /SIL Open Font License(?:,)?(?: Version)? 1\.1/,
    'NOTICE must identify the font license');
  assert.ok(notice.includes('OFL.txt') && notice.includes('PROVENANCE.md'),
    'NOTICE must identify the complete license and provenance');

  const sourceDir = 'app/ui/src/assets/fonts';
  const builtDir = 'app/ui/dist/assets';
  assert.ok(existsSync(resolve(root, sourceDir)), 'Source font directory must exist');
  assert.ok(existsSync(resolve(root, builtDir)), 'Built font directory must exist');
  const sourceFonts = readdirSync(resolve(root, sourceDir)).filter((name) => name.endsWith('.woff2'));
  const builtFonts = readdirSync(resolve(root, builtDir)).filter((name) => name.endsWith('.woff2'));
  assert.strictEqual(sourceFonts.length, 4, 'All four canonical font subsets must remain');
  assert.strictEqual(builtFonts.length, sourceFonts.length, 'All four font copies must ship');
  for (const name of sourceFonts) {
    const source = requiredFile(`${sourceDir}/${name}`);
    const copies = builtFonts.filter((built) =>
      built.startsWith(`${name.slice(0, -6)}-`) &&
      requiredFile(`${builtDir}/${built}`).equals(source));
    assert.strictEqual(copies.length, 1, `Exactly one byte-identical built copy required: ${name}`);
    assert.ok(provenance.includes(name) && provenance.includes(sha256(source)),
      `Provenance must identify the source font and its exact hash: ${name}`);
  }
});
