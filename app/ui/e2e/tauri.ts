import type { Page } from '@playwright/test';

/** How the mock answers `submit_feedback`. Everything else mirrors the Rust commands. */
export type SubmitOutcome = { ok: string } | { err: string };

export interface IpcCall {
  cmd: string;
  args: Record<string, unknown>;
}

/**
 * Inject a stand-in for the Tauri IPC bridge before the app loads, so `inTauri`
 * is true and `call()` reaches these handlers. `preview_feedback` builds a draft
 * from the payload the way `cade_feedback::Feedback::to_issue` lays it out, so
 * specs can see that edits reach the preview. Every call is recorded.
 */
export async function mockTauri(page: Page, submit: SubmitOutcome = { ok: 'https://github.com/a11smiles/cade/issues/600' }) {
  await page.addInitScript((submitOutcome: SubmitOutcome) => {
    type Fb = { kind: string; title: string; answers: { heading: string; text: string }[] };
    const prefix: Record<string, string> = { bug: '[Bug] ', 'feature-request': '[Feature] ', comment: '[Comment] ' };
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    (window as unknown as { __cadeCalls: typeof calls }).__cadeCalls = calls;
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
      metadata: { currentWindow: { label: 'main' }, currentWebview: { windowLabel: 'main', label: 'main' } },
      transformCallback: () => 0,
      invoke: async (cmd: string, args: Record<string, unknown>) => {
        calls.push({ cmd, args: JSON.parse(JSON.stringify(args ?? {})) });
        const fb = (args as { feedback?: Fb }).feedback;
        switch (cmd) {
          case 'preview_feedback':
            if (!fb || fb.title.trim() === '') throw { Message: 'a title is required' };
            return {
              title: `${prefix[fb.kind]}${fb.title.trim()}`,
              body: fb.answers.map((a) => `### ${a.heading}\n\n${a.text.trim() || '_No response_'}`).join('\n\n'),
              labels: [fb.kind === 'feature-request' ? 'enhancement' : fb.kind],
            };
          case 'submit_feedback':
            if ('err' in submitOutcome) throw submitOutcome.err;
            return submitOutcome.ok;
          default:
            throw `unknown command ${cmd}`;
        }
      },
    };
  }, submit);
}

export async function ipcCalls(page: Page, cmd: string): Promise<IpcCall[]> {
  const calls = await page.evaluate(() => (window as unknown as { __cadeCalls: IpcCall[] }).__cadeCalls);
  return calls.filter((c) => c.cmd === cmd);
}
