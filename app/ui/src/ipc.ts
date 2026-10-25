import { invoke } from '@tauri-apps/api/core';

/** True inside the CADE desktop app; false in a plain browser (`pnpm vite`). */
export const inTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!inTauri) throw new Error(`"${cmd}" needs the desktop app`);
  return invoke<T>(cmd, args);
}
