import { resolve } from 'node:path';
import ts from 'typescript';
import { describe, expect, it } from 'vitest';

// Compile against the real adapter and generated bindings, without executing IPC
// or writing fixtures/artifacts. No parallel command schema lives in this test.
function diagnostics(body: string) {
  const file = resolve('src/ipc.contract-fixture.ts');
  const options: ts.CompilerOptions = {
    strict: true, noEmit: true, skipLibCheck: true,
    target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler, types: [],
  };
  const host = ts.createCompilerHost(options);
  const original = host.getSourceFile.bind(host);
  host.getSourceFile = (name, languageVersion, onError, shouldCreateNewSourceFile) =>
    name === file
      ? ts.createSourceFile(name, `import { call } from './ipc';\n${body}`, languageVersion)
      : original(name, languageVersion, onError, shouldCreateNewSourceFile);
  const program = ts.createProgram([file], options, host);
  return ts.getPreEmitDiagnostics(program).map((diagnostic) => ({
    code: diagnostic.code,
    message: ts.flattenDiagnosticMessageText(diagnostic.messageText, '\n'),
  }));
}

describe('generated IPC compile contract', () => {
  it.each([
    ['unknown command', "call('unknown_command');", 2345],
    ['missing shell args', "call('check_shell');", 2554],
    ['missing preview args', "call('preview_feedback');", 2554],
    ['missing submit args', "call('submit_feedback');", 2554],
    ['wrong shell argument', "call('check_shell', { command: 42 });", 2322],
    ['wrong feedback argument', "call('preview_feedback', { feedback: 'wrong' });", 2322],
    ['unexpected policy argument', "call('current_policy', { command: 'git status' });", 2345],
    ['arbitrary caller result', "call<number>('check_shell', { command: 'git status' });", 2344],
  ])('rejects %s', (_name, body, code) => {
    expect(diagnostics(body)).toEqual([
      { code, message: expect.any(String) },
    ]);
  });

  it('infers all real command results and accepts their generated arguments', () => {
    expect(diagnostics(`
      import type { Feedback, IssueDraft, JsonValue } from './ipc/bindings';
      declare const feedback: Feedback;
      const policy = call('current_policy');
      const explicitUndefined = call('current_policy', undefined);
      const shell = call('check_shell', { command: 'git status' });
      const preview = call('preview_feedback', { feedback });
      const submitted = call('submit_feedback', { feedback });
      const results: [Promise<JsonValue>, Promise<JsonValue>, Promise<string>, Promise<IssueDraft>, Promise<string>] =
        [policy, explicitUndefined, shell, preview, submitted];
    `)).toEqual([]);
  });

  it.each([
    ['unresolved optional/required command without args', `
      declare const command: 'check_shell' | 'current_policy';
      call(command);
    `],
    ['unresolved optional/required command with shell args', `
      declare const command: 'check_shell' | 'current_policy';
      call(command, { command: 'a' });
    `],
    ['unresolved optional/required command with explicit undefined', `
      declare const command: 'check_shell' | 'current_policy';
      call(command, undefined);
    `],
    ['unresolved required commands with only shell args', `
      declare const command: 'check_shell' | 'preview_feedback';
      call(command, { command: 'a' });
    `],
    ['unresolved required commands with only feedback args', `
      import type { Feedback } from './ipc/bindings';
      declare const feedback: Feedback;
      declare const command: 'check_shell' | 'preview_feedback';
      call(command, { feedback });
    `],
    ['explicit command union with uncorrelated args', `
      call<'check_shell' | 'current_policy'>('check_shell');
    `],
  ])('rejects %s', (_name, body) => {
    expect(diagnostics(body)).toEqual([
      { code: 2345, message: expect.any(String) },
    ]);
  });

  it('retains narrowed command and correlated request result inference', () => {
    expect(diagnostics(`
      import type { Commands, Feedback, IssueDraft, JsonValue } from './ipc/bindings';
      declare const command: 'check_shell' | 'current_policy';
      if (command === 'check_shell') {
        const result = call(command, { command: 'a' });
        const checked: Promise<string> = result;
      } else {
        const result = call(command, undefined);
        const checked: Promise<JsonValue> = result;
      }
      type Request = {
        [C in keyof Commands]: { command: C; args: Commands[C]['args'] }
      }[keyof Commands];
      declare const request: Request;
      switch (request.command) {
        case 'current_policy': {
          const result = call(request.command, request.args);
          const checked: Promise<JsonValue> = result;
          break;
        }
        case 'check_shell':
        case 'submit_feedback': {
          if (request.command === 'check_shell') {
            const result = call(request.command, request.args);
            const checked: Promise<string> = result;
          } else {
            const result = call(request.command, request.args);
            const checked: Promise<string> = result;
          }
          break;
        }
        case 'preview_feedback': {
          const result = call(request.command, request.args);
          const checked: Promise<IssueDraft> = result;
          break;
        }
      }
    `)).toEqual([]);
  });

  it.each([
    ['shell command with feedback payload', `
      import type { Feedback } from './ipc/bindings';
      declare const feedback: Feedback;
      const args = { feedback };
      call('check_shell', args);
    `],
    ['preview command with shell payload', `
      const args = { command: 'a' };
      call('preview_feedback', args);
    `],
    ['shell command with undefined payload', "call('check_shell', undefined);"],
    ['preview command with undefined payload', "call('preview_feedback', undefined);"],
    ['submit command with undefined payload', "call('submit_feedback', undefined);"],
  ])('retains rejection of %s', (_name, body) => {
    expect(diagnostics(body)).toEqual([
      { code: 2345, message: expect.any(String) },
    ]);
  });

  it.each([
    ['policy', "call('current_policy')"],
    ['shell', "call('check_shell', { command: 'a' })"],
    ['preview', "call('preview_feedback', { feedback })"],
    ['submit', "call('submit_feedback', { feedback })"],
  ])('retains rejection of incompatible inferred %s result', (_name, expression) => {
    expect(diagnostics(`
      import type { Feedback } from './ipc/bindings';
      declare const feedback: Feedback;
      const result = ${expression};
      const incompatible: Promise<number> = result;
    `)).toEqual([
      { code: 2322, message: expect.any(String) },
    ]);
  });
});
