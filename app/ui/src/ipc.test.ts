import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

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

    const result = await call<string>('check_shell', { command: 'git status' });

    expect(result).toBe('allow');
    expect(seen).toEqual([{ cmd: 'check_shell', args: { command: 'git status' } }]);
  });

  it('call_CommandFails_RejectsWithRustError', async () => {
    mockIPC(() => {
      throw 'no GitHub token stored';
    });
    const { call } = await loadIpc();

    await expect(call('submit_feedback', {})).rejects.toBe('no GitHub token stored');
  });
});
