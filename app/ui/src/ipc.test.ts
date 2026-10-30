import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { Feedback } from './ipc/bindings';

const feedback: Feedback = {
  kind: 'comment', title: 'Synthetic feedback', answers: { comment: 'Example' }, environment: {},
};

// `inTauri` is read once at import, so each test imports a fresh copy of the module.
async function loadIpc() {
  vi.resetModules();
  return import('./ipc');
}

beforeEach(() => {
  delete (window as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
});

afterEach(() => {
  clearMocks();
});

describe('ipc', () => {
  it('inTauri_PlainBrowser_IsFalse', async () => {
    const { inTauri } = await loadIpc();

    expect(inTauri).toBe(false);
  });

  it('inTauri_TauriInternalsPresent_IsTrue', async () => {
    mockIPC(() => null);

    const { inTauri } = await loadIpc();

    expect(inTauri).toBe(true);
  });

  it('call_PlainBrowser_RejectsNamingTheCommand', async () => {
    const { call } = await loadIpc();

    await expect(call('current_policy')).rejects.toThrow('"current_policy" needs the desktop app');
  });

  it('call_InTauri_InvokesCommandWithArgsAndReturnsResult', async () => {
    const seen: { cmd: string; args: unknown }[] = [];
    mockIPC((cmd, args) => {
      seen.push({ cmd, args });
      return 'allow';
    });
    const { call } = await loadIpc();

    const result = await call('check_shell', { command: 'git status' });

    expect(result).toBe('allow');
    expect(seen).toEqual([{ cmd: 'check_shell', args: { command: 'git status' } }]);
  });

  it('call_CommandFails_RejectsWithRustError', async () => {
    mockIPC(() => {
      throw 'no GitHub token stored';
    });
    const { call } = await loadIpc();

    await expect(call('submit_feedback', { feedback })).rejects.toBe('no GitHub token stored');
  });

  it('call_NoCallerArgs_ForwardsTheSdkDefaultAndReturnsJsonUnchanged', async () => {
    const policy = { shell: { default: 'ask' }, revision: 1, enabled: true, extra: null };
    const seen: unknown[] = [];
    mockIPC((cmd, args) => {
      seen.push({ cmd, args });
      return policy;
    });
    const { call } = await loadIpc();

    expect(await call('current_policy')).toBe(policy);
    expect(seen).toEqual([{ cmd: 'current_policy', args: {} }]);
  });

  it('call_FeedbackPreview_ForwardsTheExactArgumentObjectAndDraft', async () => {
    const args = { feedback };
    const draft = { title: '[Comment] Synthetic feedback', body: 'Example', labels: ['comment'] };
    mockIPC((cmd, received) => {
      expect(cmd).toBe('preview_feedback');
      expect(received).toBe(args);
      return draft;
    });
    const { call } = await loadIpc();

    expect(await call('preview_feedback', args)).toBe(draft);
  });

  it('call_CommandMessageError_RejectsWithTheSameNativeEnvelope', async () => {
    const error = { Message: 'Synthetic native failure' };
    mockIPC(() => { throw error; });
    const { call } = await loadIpc();

    await expect(call('submit_feedback', { feedback })).rejects.toBe(error);
  });
});
