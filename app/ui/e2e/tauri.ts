import { execFile } from 'node:child_process';
import { isAbsolute } from 'node:path';
import type { Page } from '@playwright/test';
import type { Feedback, IssueDraft } from '../src/ipc/bindings';

/** How the mock answers `submit_feedback`. Preview always runs real Rust. */
export type SubmitOutcome = { ok: string } | { err: string };

type PreviewOutcome = { ok: IssueDraft } | { error: { Message: string } };
type ControlledOutcome = { preview: true } | { ok: string } | { err: { Message: string } };

export interface IpcCall {
  cmd: string;
  args: Record<string, unknown>;
  preview?: PreviewOutcome;
}

/** Private Node-only bridge: no webview network, shell permission or JS business rules. */
function rustPreview(binary: string, feedback: Feedback): Promise<PreviewOutcome> {
  return new Promise((resolve, reject) => {
    const child = execFile(binary, ['preview'], { timeout: 15_000, maxBuffer: 2 * 1024 * 1024 }, (error, stdout) => {
      if (error) {
        reject(new Error('The Rust feedback preview process failed.'));
        return;
      }
      try {
        resolve(JSON.parse(stdout) as PreviewOutcome);
      } catch {
        reject(new Error('The Rust feedback preview returned invalid JSON.'));
      }
    });
    child.stdin?.on('error', () => reject(new Error('Could not write the Rust feedback preview input.')));
    child.stdin?.end(JSON.stringify(feedback));
  });
}

/**
 * Stand in for Tauri transport, not feedback logic. Successful previews come
 * exclusively from generate_feedback. Controlled calls wrap the real response
 * with explicit release/reordering or a synthetic transport failure.
 */
export async function mockTauri(
  page: Page,
  submit: SubmitOutcome = { ok: 'https://github.com/a11smiles/cade/issues/600' },
  controlledCommands: string[] = [],
) {
  const binary = process.env.CADE_FEEDBACK_PREVIEW_BIN;
  if (!binary || !isAbsolute(binary)) throw new Error('Global setup must provide an absolute CADE_FEEDBACK_PREVIEW_BIN.');
  await page.exposeBinding('__cadeRustPreview', (_source, feedback: Feedback) => rustPreview(binary, feedback));
  await page.addInitScript(({ submitOutcome, controlledCommands }: { submitOutcome: SubmitOutcome; controlledCommands: string[] }) => {
    const calls: IpcCall[] = [];
    const pending = new Map<string, (outcome: ControlledOutcome) => Promise<PreviewOutcome | undefined>>();
    const bridge = window as unknown as {
      __cadeCalls: IpcCall[];
      __cadeRustPreview: (feedback: Feedback) => Promise<PreviewOutcome>;
      __cadeRespond: (cmd: string, index: number, outcome: ControlledOutcome) => Promise<PreviewOutcome | undefined>;
      __TAURI_INTERNALS__: unknown;
    };
    bridge.__cadeCalls = calls;
    bridge.__cadeRespond = async (cmd, index, outcome) => {
      const key = `${cmd}:${index}`;
      const request = pending.get(key);
      if (!request) throw new Error(`No controlled IPC request ${key}`);
      pending.delete(key);
      return request(outcome);
    };
    bridge.__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' }, currentWebview: { windowLabel: 'main', label: 'main' } },
      transformCallback: () => 0,
      invoke: (cmd: string, args: Record<string, unknown>) => {
        const index = calls.filter((call) => call.cmd === cmd).length;
        const call: IpcCall = { cmd, args: JSON.parse(JSON.stringify(args ?? {})) };
        calls.push(call);
        return new Promise((resolve, reject) => {
          const reply = async (outcome?: ControlledOutcome): Promise<PreviewOutcome | undefined> => {
            try {
              if (cmd === 'preview_feedback') {
                const result = await bridge.__cadeRustPreview(call.args.feedback as Feedback);
                call.preview = result;
                if (outcome && 'err' in outcome) reject(outcome.err);
                else if ('error' in result) reject(result.error);
                else resolve(result.ok);
                return result;
              }
              if (outcome && 'err' in outcome) reject(outcome.err);
              else if (outcome && 'ok' in outcome) resolve(outcome.ok);
              else if (cmd === 'submit_feedback') {
                if ('err' in submitOutcome) reject(submitOutcome.err);
                else resolve(submitOutcome.ok);
              } else reject(`unknown command ${cmd}`);
            } catch (error) {
              reject(error);
              // Fail the test's explicit release too; never substitute a draft.
              throw error;
            }
          };
          if (controlledCommands.includes(cmd)) pending.set(`${cmd}:${index}`, reply);
          else void reply().catch(() => { /* invoke's promise already carries the failure */ });
        });
      },
    };
  }, { submitOutcome: submit, controlledCommands });
}

export async function ipcCalls(page: Page, cmd: string): Promise<IpcCall[]> {
  const calls = await page.evaluate(() => (window as unknown as { __cadeCalls: IpcCall[] }).__cadeCalls);
  return calls.filter((c) => c.cmd === cmd);
}
