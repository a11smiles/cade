import { spawn } from 'node:child_process';
import { constants } from 'node:fs';
import { access } from 'node:fs/promises';
import { isAbsolute } from 'node:path';
import { createInterface } from 'node:readline';
import { fileURLToPath } from 'node:url';

/** Build only the preview example, once per suite; never generate tracked files. */
async function buildPreview(): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = spawn('cargo', [
      'build', '--locked', '-p', 'cade-feedback', '--example', 'generate_feedback',
      '--message-format=json', '--jobs=2',
    ], {
      cwd: fileURLToPath(new URL('../../../', import.meta.url)),
      env: process.env,
      stdio: ['ignore', 'pipe', 'inherit'],
    });
    let executable: string | undefined;
    const lines = createInterface({ input: child.stdout });
    lines.on('line', (line) => {
      const artifact = JSON.parse(line) as {
        reason?: string;
        target?: { name?: string; kind?: string[] };
        executable?: string;
      };
      if (artifact.reason === 'compiler-artifact' && artifact.target?.name === 'generate_feedback' &&
        artifact.target.kind?.includes('example') && artifact.executable) executable = artifact.executable;
    });
    child.on('error', reject);
    child.on('close', (code) => {
      if (code === 0 && executable) resolve(executable);
      else reject(new Error(`Could not build the Rust feedback preview example (exit ${code}).`));
    });
  });
}

export default async function globalSetup() {
  // Presence is deliberate: an invalid supplied path must fail, not trigger a fallback.
  const binary = process.env.CADE_FEEDBACK_PREVIEW_BIN ?? await buildPreview();
  if (!isAbsolute(binary)) throw new Error('CADE_FEEDBACK_PREVIEW_BIN must be an absolute executable path.');
  await access(binary, constants.X_OK);
  process.env.CADE_FEEDBACK_PREVIEW_BIN = binary;
}
