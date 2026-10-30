import { invoke } from '@tauri-apps/api/core';
import type { Commands } from './ipc/bindings';

/** True inside the CADE desktop app; false in a plain browser (`pnpm vite`). */
export const inTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export async function call<C extends keyof Commands>(
  // An unresolved command union cannot establish which argument tuple is safe.
  // Require a single command; narrow correlated requests before calling.
  cmd: C & { [K in C]: [C] extends [K] ? K : never }[C],
  ...[args]: C extends keyof Commands
    ? Commands[C]['args'] extends undefined
      ? [args?: Commands[C]['args']]
      : [args: Commands[C]['args']]
    : never
): Promise<Commands[C]['result']> {
  if (!inTauri) throw new Error(`"${cmd}" needs the desktop app`);
  return invoke<Commands[C]['result']>(cmd, args);
}
