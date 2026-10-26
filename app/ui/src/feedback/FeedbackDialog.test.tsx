import { act, render, screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { FeedbackDialog } from './FeedbackDialog';
import { FORMS as FORM_METADATA } from './forms.generated';
import type { Feedback, IssueDraft } from './types';

const ipc = vi.hoisted(() => ({ inTauri: true, call: vi.fn() }));
vi.mock('../ipc', () => ({
  get inTauri() {
    return ipc.inTauri;
  },
  call: ipc.call,
}));

const DRAFT: IssueDraft = { title: '[Bug] Canvas freezes', body: '### What happened?\n\nIt froze.', labels: ['bug', 'from-app'] };

function previewCalls(): Feedback[] {
  return ipc.call.mock.calls.filter(([cmd]) => cmd === 'preview_feedback').map(([, args]) => (args as { feedback: Feedback }).feedback);
}

function lastPreview(): Feedback {
  const calls = previewCalls();
  const last = calls[calls.length - 1];
  if (!last) throw new Error('preview_feedback was never called');
  return last;
}

async function fillBug() {
  await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), 'Canvas freezes');
  await userEvent.type(screen.getByRole('textbox', { name: /What happened\?/ }), 'It froze.');
  await userEvent.type(screen.getByRole('textbox', { name: /Steps to reproduce/ }), 'Open a big file.');
}

function holdFeedbackRequests() {
  const pending: {
    cmd: string;
    feedback: Feedback;
    resolve: (value: IssueDraft | string) => void;
    reject: (error: { Message: string }) => void;
  }[] = [];
  ipc.call.mockImplementation((cmd: string, args: { feedback: Feedback }) =>
    new Promise<IssueDraft | string>((resolve, reject) => {
      pending.push({ cmd, feedback: args.feedback, resolve, reject });
    }),
  );
  return pending;
}

beforeEach(() => {
  ipc.inTauri = true;
  ipc.call.mockReset();
  ipc.call.mockImplementation((cmd: string) => (cmd === 'preview_feedback' ? Promise.resolve(DRAFT) : new Promise(() => {})));
});

describe('FeedbackDialog', () => {
  it('FeedbackDialog_Opened_IsModalDialogWithBugSelected', () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    expect(screen.getByRole('dialog', { name: 'Send feedback' })).toHaveAttribute('aria-modal', 'true');
    expect(screen.getByRole('radio', { name: /^Bug/ })).toHaveAttribute('aria-checked', 'true');
    expect(screen.getByRole('textbox', { name: /What happened\? required/ })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /Steps to reproduce required/ })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: /What did you expect\? optional/ })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
  });

  // Assertion reds are recorded in feedback-ui-parity-red/RESULTS.md.
  describe('FeedbackDialog form parity', () => {
    const kindNames = { bug: /^Bug/, 'feature-request': /^Feature request/, comment: /^Comment/ };
    const environmentKeys = ['app_version', 'os', 'gpu', 'policy_version'];

    for (const kind of ['bug', 'feature-request', 'comment'] as const) {
      for (const field of FORM_METADATA[kind].body) {
        if (!('id' in field) || environmentKeys.includes(field.id) || field.id === 'screenshot_url') continue;

        it(`FeedbackDialog_${kind}-${field.id}Metadata_RendersCanonicalControlAndOptions`, async () => {
          render(<FeedbackDialog onClose={vi.fn()} />);
          await userEvent.click(screen.getByRole('radio', { name: kindNames[kind] }));
          const multiple = 'multiple' in field.attributes && field.attributes.multiple;
          const role = field.type === 'dropdown' ? (multiple ? 'listbox' : 'combobox') : 'textbox';
          const control = screen.getByRole(role, { name: (name) => name.startsWith(field.attributes.label) });

          expect.soft(control).toHaveAttribute('name', field.id);
          expect(control.tagName).toBe(field.type === 'textarea' ? 'TEXTAREA' : field.type === 'dropdown' ? 'SELECT' : 'INPUT');
          if ('validations' in field && field.validations.required) expect(control).toBeRequired();
          else expect(control).not.toBeRequired();
          if ('options' in field.attributes) {
            expect(within(control).getAllByRole('option').map((option) => option.textContent)).toEqual(
              multiple ? [...field.attributes.options] : ['Not specified', ...field.attributes.options],
            );
            expect(control).toHaveValue(multiple ? [] : '');
          }
        });
      }

      it(`FeedbackDialog_${kind}ExistingTextAnswers_UsesCanonicalKeysInsteadOfLegacyHeadings`, async () => {
        render(<FeedbackDialog onClose={vi.fn()} />);
        await userEvent.click(screen.getByRole('radio', { name: kindNames[kind] }));
        await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), 'Synthetic keyed report');
        const expected: Record<string, string> = {};

        for (const field of FORM_METADATA[kind].body) {
          if (!('id' in field) || environmentKeys.includes(field.id) || field.id === 'screenshot_url') continue;
          expected[field.id] = '';
          if (!['what-happened', 'steps', 'expected', 'problem', 'proposal', 'comment', 'where'].includes(field.id)) continue;
          const value = `Synthetic ${field.id}`;
          await userEvent.type(screen.getByRole('textbox', { name: (name) => name.startsWith(field.attributes.label) }), value);
          expected[field.id] = value;
        }

        // All controls used above exist on the legacy baseline: this assertion
        // independently reproduces the array-versus-keyed-payload defect.
        await waitFor(() => expect(lastPreview().answers).toEqual(expected));
      });

      it(`FeedbackDialog_${kind}Answers_UsesCanonicalPayloadKeysAndOptionOrder`, async () => {
        render(<FeedbackDialog onClose={vi.fn()} />);
        await userEvent.click(screen.getByRole('radio', { name: kindNames[kind] }));
        await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), 'Synthetic report');
        const expected: Record<string, string> = {};

        for (const field of FORM_METADATA[kind].body) {
          if (!('id' in field) || environmentKeys.includes(field.id) || field.id === 'screenshot_url') continue;
          const name = (label: string) => label.startsWith(field.attributes.label);
          if ('options' in field.attributes) {
            const multiple = 'multiple' in field.attributes && field.attributes.multiple;
            if (multiple) {
              await userEvent.selectOptions(screen.getByRole('listbox', { name }), ['Image', 'App']);
              expected[field.id] = 'App, Image';
            } else {
              const value = field.attributes.options[0];
              await userEvent.selectOptions(screen.getByRole('combobox', { name }), value);
              expected[field.id] = value;
            }
          } else {
            const value = `Synthetic ${field.id}`;
            await userEvent.type(screen.getByRole('textbox', { name }), value);
            expected[field.id] = value;
          }
        }

        await waitFor(() => expect(lastPreview().answers).toEqual(expected));
        expect(lastPreview().kind).toBe(kind);
      });

      it(`FeedbackDialog_${kind}UnansweredOptionalChoices_SendsEmptyValuesNotInventedSelections`, async () => {
        render(<FeedbackDialog onClose={vi.fn()} />);
        await userEvent.click(screen.getByRole('radio', { name: kindNames[kind] }));
        const expected = Object.fromEntries(FORM_METADATA[kind].body.flatMap((field) =>
          'id' in field && 'options' in field.attributes && !environmentKeys.includes(field.id) ? [[field.id, '']] : [],
        ));

        await waitFor(() => expect(lastPreview().answers).toMatchObject(expected));
      });
    }

    it('FeedbackDialog_VersionControl_UsesExistingAppVersionKeyNotLegacyVersion', async () => {
      render(<FeedbackDialog onClose={vi.fn()} />);
      const version = screen.getByRole('checkbox', { name: 'CADE version' });

      expect.soft(version).toHaveAttribute('name', 'app_version');
      await waitFor(() => expect(lastPreview().environment.app_version).toBe('0.1.0'));
      expect(lastPreview().environment).not.toHaveProperty('version');
      await userEvent.click(version);
      await waitFor(() => expect(lastPreview().environment.app_version).toBeNull());
      expect(lastPreview().environment).not.toHaveProperty('version');
    });

    it.each([
      ['GPU and renderer', 'gpu'],
      ['Policy version', 'policy_version'],
    ] as const)('FeedbackDialog_%sUnavailable_ExplainsOmissionAndNeverInventsAValue', async (label, key) => {
      render(<FeedbackDialog onClose={vi.fn()} />);
      const control = screen.getByRole('checkbox', { name: label });

      expect.soft(control).toBeDisabled();
      expect.soft(control).not.toBeChecked();
      expect.soft(control).toHaveAccessibleDescription('Not available. This detail will not be sent.');
      await waitFor(() => expect(lastPreview().environment[key]).toBeNull());
    });

    it('FeedbackDialog_ScreenshotUrlEdited_UsesExistingTopLevelPayloadKey', async () => {
      render(<FeedbackDialog onClose={vi.fn()} />);
      const screenshot = screen.getByRole('textbox', { name: /Screenshot URL/ });
      const url = 'https://example.com/synthetic-feedback.png';

      expect(screenshot).toHaveAttribute('name', 'screenshot_url');
      await userEvent.type(screenshot, url);

      await waitFor(() => expect(lastPreview().screenshot_url).toBe(url));
      expect(lastPreview().answers).not.toHaveProperty('screenshot_url');
      await userEvent.clear(screenshot);
      await waitFor(() => expect(lastPreview().screenshot_url ?? null).toBeNull());
    });
  });

  describe('FeedbackDialog next-phase preview contracts', () => {
    it('FeedbackDialog_RustDraftLabels_RendersEveryLabelVerbatimAndInOrder', async () => {
      const draft: IssueDraft = { title: 'Rust title', body: 'Rust body\n\nUnchanged.', labels: ['type:bug', 'needs-triage', 'bug'] };
      ipc.call.mockResolvedValue(draft);
      render(<FeedbackDialog onClose={vi.fn()} />);

      expect((await screen.findByLabelText('Issue preview')).textContent).toBe(`${draft.title}\n\n${draft.body}`);
      expect(screen.getByLabelText('Labels').textContent).toBe(draft.labels.join(', '));
    });

    it('FeedbackDialog_SubmitPending_DisablesEditingAndKeepsReviewedEntries', async () => {
      render(<FeedbackDialog onClose={vi.fn()} />);
      await fillBug();
      const reviewed = lastPreview();
      const pending = holdFeedbackRequests();
      await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));

      for (const role of ['textbox', 'radio', 'checkbox', 'listbox'] as const) {
        for (const control of screen.getAllByRole(role)) expect.soft(control).toBeDisabled();
      }
      await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), ' Must not replace reviewed content');
      expect(screen.getByRole('textbox', { name: 'Title' })).toHaveValue(reviewed.title);
      expect(pending.filter(({ cmd }) => cmd === 'submit_feedback').map(({ feedback }) => feedback)).toEqual([reviewed]);
      expect(screen.getByRole('button', { name: 'Sending…' })).toBeDisabled();
    });

    it('FeedbackDialog_SubmitRejected_ExplainsPreservedEntriesWithoutClaimingNotSent', async () => {
      render(<FeedbackDialog onClose={vi.fn()} />);
      await fillBug();
      const pending = holdFeedbackRequests();
      await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));
      const submission = pending.find(({ cmd }) => cmd === 'submit_feedback');
      if (!submission) throw new Error('No feedback submission was captured');
      await act(async () => submission.reject({ Message: 'Synthetic response interrupted' }));

      expect(screen.getByText('Couldn’t submit your issue. Your entries are still in this form.')).toBeInTheDocument();
      expect(screen.queryByText(/not sent/i)).not.toBeInTheDocument();
      expect(screen.getByRole('textbox', { name: 'Title' })).toHaveValue('Canvas freezes');
      expect(screen.getByRole('textbox', { name: /What happened\?/ })).toHaveValue('It froze.');
      expect(screen.getByRole('textbox', { name: /Steps to reproduce/ })).toHaveValue('Open a big file.');
      expect(screen.getByRole('textbox', { name: 'Title' })).toBeEnabled();
    });

    it('FeedbackDialog_LogsUnavailable_UsesApprovedNoLogsWillBeSentCopy', () => {
      render(<FeedbackDialog onClose={vi.fn()} />);
      const logs = screen.getByRole('checkbox', { name: /Recent logs, scrubbed/ });

      expect(logs).toBeDisabled();
      expect(logs).not.toBeChecked();
      expect(logs).toHaveAccessibleDescription('Recent logs are not available. No logs will be sent.');
    });
  });

  it.each([
    [/^Feature request/, 'Submit request', ['What problem would this solve?', 'What would you like CADE to do?', 'Which output type?', 'What do you do today instead?', 'Who would benefit?', 'Would you like to contribute?']],
    [/^Comment/, 'Send comment', ['Your comment', 'Where in CADE?', 'Output type', 'Would you like a reply?']],
  ])('FeedbackDialog_KindChosen_%s_ShowsThatFormsQuestions', async (kind, cta, headings) => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    await userEvent.click(screen.getByRole('radio', { name: kind }));

    expect(screen.getByRole('radio', { name: kind })).toHaveAttribute('aria-checked', 'true');
    const questions = [
      ...screen.getAllByRole('textbox').filter((control) => !['fb-t', 'fb-screenshot'].includes(control.id)),
      ...screen.getAllByRole('listbox'),
      ...screen.getAllByRole('combobox'),
    ];
    expect(questions.map((control) => control.id)).toHaveLength(headings.length);
    for (const h of headings) expect(screen.getByText(h)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: cta })).toBeInTheDocument();
  });

  it('FeedbackDialog_KindChanged_ClearsAnswersButKeepsTitle', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();

    await userEvent.click(screen.getByRole('radio', { name: /^Comment/ }));

    expect(screen.getByRole('textbox', { name: 'Title' })).toHaveValue('Canvas freezes');
    expect(screen.getByRole('textbox', { name: /Your comment/ })).toHaveValue('');
  });

  it('FeedbackDialog_RequiredAnswersFilled_EnablesSubmitWithoutOptionalOnes', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    await fillBug();

    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeEnabled();
  });

  it.each([
    ['Title', 'Title'],
    ['first required answer', /What happened\?/],
  ])('FeedbackDialog_Missing_%s_KeepsSubmitDisabled', async (_, field) => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();

    await userEvent.clear(screen.getByRole('textbox', { name: field }));

    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
  });

  it('FeedbackDialog_WhitespaceOnlyTitle_KeepsSubmitDisabled', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();

    await userEvent.clear(screen.getByRole('textbox', { name: 'Title' }));
    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), '   ');

    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
  });

  it('FeedbackDialog_FormEdited_SendsRustShapedFeedbackForPreview', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    await fillBug();

    await waitFor(() => expect(lastPreview().title).toBe('Canvas freezes'));
    const fb = lastPreview();
    expect(fb.kind).toBe('bug');
    expect(fb.answers).toEqual({
      'what-happened': 'It froze.',
      steps: 'Open a big file.',
      expected: '',
      actual: '',
      output: '',
      area: '',
      severity: '',
    });
    expect(fb.environment.app_version).toBe('0.1.0');
    expect(fb.environment.os).toBe(navigator.userAgent);
    expect(fb.environment.logs).toBeNull();
  });

  it('FeedbackDialog_FeatureRequest_SendsKebabCaseKind', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    await userEvent.click(screen.getByRole('radio', { name: /^Feature request/ }));

    await waitFor(() => expect(lastPreview().kind).toBe('feature-request'));
  });

  it.each([
    ['CADE version', 'app_version'],
    ['Operating system', 'os'],
    ['GPU and renderer', 'gpu'],
    ['Policy version', 'policy_version'],
  ] as const)('FeedbackDialog_EnvironmentOptOut_%s_SendsNull', async (label, key) => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    const control = screen.getByRole('checkbox', { name: label });
    if (key === 'gpu' || key === 'policy_version') {
      expect(control).toBeDisabled();
      expect(control).not.toBeChecked();
      await waitFor(() => expect(lastPreview().environment[key]).toBeNull());
    } else {
      await waitFor(() => expect(lastPreview().environment[key]).not.toBeNull());
    }

    await userEvent.click(control);

    await waitFor(() => expect(lastPreview().environment[key]).toBeNull());
  });

  it('FeedbackDialog_PreviewReturned_ShowsExactlyWhatGitHubWillReceive', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    const preview = await screen.findByLabelText('Issue preview');

    expect(preview.textContent).toBe('[Bug] Canvas freezes\n\n### What happened?\n\nIt froze.');
  });

  it('FeedbackDialog_PreviewFails_ShowsNoPreview', async () => {
    ipc.call.mockImplementation(() => Promise.reject('redaction failed'));
    render(<FeedbackDialog onClose={vi.fn()} />);

    await waitFor(() => expect(previewCalls().length).toBeGreaterThan(0));

    expect(screen.queryByLabelText('Issue preview')).toBeNull();
  });

  it('FeedbackDialog_Submitting_ShowsSendingAndDisablesSubmit', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();

    await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));

    expect(screen.getByRole('button', { name: 'Sending…' })).toBeDisabled();
    expect(ipc.call).toHaveBeenCalledWith('submit_feedback', { feedback: lastPreview() });
  });

  it('FeedbackDialog_CurrentEditPreviewPending_DisablesSubmitWithoutSending', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();
    expect(await screen.findByLabelText('Issue preview')).toHaveTextContent(DRAFT.title);
    holdFeedbackRequests();

    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), ' New');

    expect(lastPreview().title).toBe('Canvas freezes New');
    const submit = screen.getByRole('button', { name: 'Submit bug' });
    // Keep both assertions observable on the broken baseline.
    expect.soft(submit).toBeDisabled();
    await userEvent.click(submit);
    expect(ipc.call.mock.calls.filter(([cmd]) => cmd === 'submit_feedback')).toHaveLength(0);
  });

  it('FeedbackDialog_CurrentPreviewRejected_DisablesSubmitAndKeepsEntries', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();
    await screen.findByLabelText('Issue preview');
    const pending = holdFeedbackRequests();
    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), ' New');
    const current = pending[pending.length - 1];
    if (!current) throw new Error('The edited feedback did not request a preview');

    await act(async () => current.reject({ Message: 'Feedback preview is temporarily unavailable' }));

    expect(screen.getByRole('textbox', { name: 'Title' })).toHaveValue('Canvas freezes New');
    expect(screen.getByRole('textbox', { name: /What happened\?/ })).toHaveValue('It froze.');
    expect(screen.getByRole('textbox', { name: /Steps to reproduce/ })).toHaveValue('Open a big file.');
    const submit = screen.getByRole('button', { name: 'Submit bug' });
    expect.soft(submit).toBeDisabled();
    await userEvent.click(submit);
    expect(ipc.call.mock.calls.filter(([cmd]) => cmd === 'submit_feedback')).toHaveLength(0);
  });

  it('FeedbackDialog_CurrentPreviewRejected_OffersExplicitRetryForTheSameEntries', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();
    await screen.findByLabelText('Issue preview');
    const pending = holdFeedbackRequests();
    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), ' New');
    const failed = pending[pending.length - 1];
    if (!failed) throw new Error('The edited feedback did not request a preview');
    await act(async () => failed.reject({ Message: 'Feedback preview is temporarily unavailable' }));

    expect.soft(screen.queryByText('Couldn’t preview your issue. Your entries are still in this form.')).toBeInTheDocument();
    const retry = screen.getByRole('button', { name: 'Retry preview' });
    const count = previewCalls().length;
    await userEvent.click(retry);

    expect(previewCalls()).toHaveLength(count + 1);
    expect(lastPreview()).toEqual(failed.feedback);
    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
    const retried = pending[pending.length - 1];
    if (!retried) throw new Error('Retry did not request a preview');
    const draft = { ...DRAFT, title: '[Bug] Canvas freezes New' };
    await act(async () => retried.resolve(draft));
    expect(screen.getByLabelText('Issue preview').textContent).toBe(`${draft.title}\n\n${draft.body}`);
    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeEnabled();
    expect(ipc.call.mock.calls.filter(([cmd]) => cmd === 'submit_feedback')).toHaveLength(0);
  });

  it('FeedbackDialog_EditDuringSubmit_KeepsTheDisplayedPreviewInputSnapshot', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();
    const reviewed = JSON.parse(JSON.stringify(lastPreview())) as Feedback;
    expect(screen.getByLabelText('Issue preview').textContent).toBe(`${DRAFT.title}\n\n${DRAFT.body}`);
    const pending = holdFeedbackRequests();

    await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));
    const submission = pending.find(({ cmd }) => cmd === 'submit_feedback');
    expect(submission?.feedback).toEqual(reviewed);
    expect(screen.getByRole('textbox', { name: 'Title' })).toBeDisabled();
    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), ' New');

    expect(screen.getByRole('textbox', { name: 'Title' })).toHaveValue(reviewed.title);
    expect(submission?.feedback).toEqual(reviewed);
    expect(pending.filter(({ cmd }) => cmd === 'submit_feedback')).toHaveLength(1);
    expect(screen.getByRole('button', { name: 'Sending…' })).toBeDisabled();
  });

  it('FeedbackDialog_SubmitSucceeds_FocusesSuccessHeadingBeforeAnotherKeypress', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();
    const pending = holdFeedbackRequests();
    await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));
    const submission = pending.find(({ cmd }) => cmd === 'submit_feedback');
    if (!submission) throw new Error('No feedback submission was captured');

    await act(async () => submission.resolve('https://github.com/a11smiles/cade/issues/600'));

    // No Tab or manual focus call may repair the transition before these checks.
    expect.soft(screen.getByRole('dialog')).toContainElement(document.activeElement as HTMLElement);
    const heading = screen.getByRole('heading', { name: 'Issue opened' });
    expect(heading).toHaveFocus();
    expect(screen.getByRole('status')).toContainElement(heading);
  });

  it('FeedbackDialog_SubmitSucceeds_ShowsLinkToOpenedIssue', async () => {
    const onClose = vi.fn();
    ipc.call.mockImplementation((cmd: string) =>
      Promise.resolve(cmd === 'preview_feedback' ? DRAFT : 'https://github.com/a11smiles/cade/issues/600'),
    );
    render(<FeedbackDialog onClose={onClose} />);
    await fillBug();

    await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));

    const status = await screen.findByRole('status');
    expect(status).toHaveTextContent('Issue opened');
    expect(screen.getByRole('link', { name: 'https://github.com/a11smiles/cade/issues/600' })).toHaveAttribute(
      'href',
      'https://github.com/a11smiles/cade/issues/600',
    );
    await userEvent.click(within(status).getByRole('button', { name: 'Close' }));
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('FeedbackDialog_SubmitFails_ShowsErrorAndKeepsTheForm', async () => {
    ipc.call.mockImplementation((cmd: string) => (cmd === 'preview_feedback' ? Promise.resolve(DRAFT) : Promise.reject('no GitHub token stored')));
    render(<FeedbackDialog onClose={vi.fn()} />);
    await fillBug();

    await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));

    expect(await screen.findByRole('alert')).toHaveTextContent('no GitHub token stored');
    expect(screen.getByRole('textbox', { name: 'Title' })).toHaveValue('Canvas freezes');
    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeEnabled();
  });

  it.each(['Close', 'Cancel'])('FeedbackDialog_%sClicked_CallsOnClose', async (name) => {
    const onClose = vi.fn();
    render(<FeedbackDialog onClose={onClose} />);

    await userEvent.click(screen.getByRole('button', { name }));

    expect(onClose).toHaveBeenCalledOnce();
  });

  it('FeedbackDialog_LogsOption_IsDisabledAndSaysItIsUnavailable', () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    const logs = screen.getByRole('checkbox', { name: /Recent logs, scrubbed/ });
    expect(logs).toBeDisabled();
    expect(logs).not.toBeChecked();
    expect(logs).toHaveAccessibleDescription('Recent logs are not available. No logs will be sent.');
  });

  it('FeedbackDialog_Opened_MovesFocusIntoTheDialog', () => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    expect(screen.getByRole('dialog')).toContainElement(document.activeElement as HTMLElement);
    expect(screen.getByRole('button', { name: 'Close' })).toHaveFocus();
  });

  it('FeedbackDialog_TabFromLastControl_WrapsToFirst', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    screen.getByRole('button', { name: 'Cancel' }).focus();

    await userEvent.tab();

    expect(screen.getByRole('button', { name: 'Close' })).toHaveFocus();
  });

  it('FeedbackDialog_ShiftTabFromFirstControl_WrapsToLast', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    screen.getByRole('button', { name: 'Close' }).focus();

    await userEvent.tab({ shift: true });

    expect(screen.getByRole('button', { name: 'Cancel' })).toHaveFocus();
  });

  it('FeedbackDialog_TabThroughEveryControl_NeverLeavesTheDialog', async () => {
    render(
      <>
        <button type="button">Behind the dialog</button>
        <FeedbackDialog onClose={vi.fn()} />
      </>,
    );

    for (let i = 0; i < 20; i++) {
      await userEvent.tab();
      expect(screen.getByRole('dialog')).toContainElement(document.activeElement as HTMLElement);
    }
  });

  it('FeedbackDialog_EscapePressed_CallsOnClose', async () => {
    const onClose = vi.fn();
    render(<FeedbackDialog onClose={onClose} />);

    await userEvent.keyboard('{Escape}');

    expect(onClose).toHaveBeenCalledOnce();
  });

  it('FeedbackDialog_Unmounted_ReturnsFocusToTheOpener', () => {
    render(<button type="button">Send feedback</button>);
    const opener = screen.getByRole('button', { name: 'Send feedback' });
    opener.focus();

    const dialog = render(<FeedbackDialog onClose={vi.fn()} />);
    expect(opener).not.toHaveFocus();
    dialog.unmount();

    expect(opener).toHaveFocus();
  });

  it('FeedbackDialog_PreviewResponsesOutOfOrder_ShowsTheLatestEdit', async () => {
    const pending: { title: string; resolve: (d: IssueDraft) => void }[] = [];
    ipc.call.mockImplementation(
      (cmd: string, args: { feedback: Feedback }) =>
        cmd === 'preview_feedback' ? new Promise<IssueDraft>((resolve) => pending.push({ title: args.feedback.title, resolve })) : new Promise(() => {}),
    );
    render(<FeedbackDialog onClose={vi.fn()} />);
    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), 'ab');
    await waitFor(() => expect(pending.map((p) => p.title)).toContain('ab'));

    const draft = (title: string): IssueDraft => ({ title: `[Bug] ${title}`, body: '', labels: ['bug'] });
    pending.filter((p) => p.title === 'ab').forEach((p) => p.resolve(draft('ab')));
    await screen.findByText('[Bug] ab');
    pending.filter((p) => p.title === 'a').forEach((p) => p.resolve(draft('a')));
    await new Promise((r) => setTimeout(r, 20));

    expect(screen.getByLabelText('Issue preview').textContent).toBe('[Bug] ab\n\n');
  });

  it('FeedbackDialog_OutsideTauri_CannotSendAndSaysWhere', async () => {
    ipc.inTauri = false;
    render(<FeedbackDialog onClose={vi.fn()} />);

    await fillBug();

    expect(screen.getByText('Open the desktop app to send.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
    expect(ipc.call).not.toHaveBeenCalled();
  });
  it('FeedbackDialog_TitleTyped_RequestsOnePreviewPerDistinctEdit', async () => {
    render(<FeedbackDialog onClose={vi.fn()} />);
    await waitFor(() => expect(previewCalls().length).toBeGreaterThan(0));
    const before = previewCalls().length;

    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), 'ab');

    await waitFor(() => expect(previewCalls().length).toBe(before + 2));
    expect(previewCalls().map((f) => f.title).slice(-2)).toEqual(['a', 'ab']);
  });

  it('FeedbackDialog_PreviewsResolveOutOfOrder_ShowsLatestPreview', async () => {
    const resolvers: ((d: IssueDraft) => void)[] = [];
    ipc.call.mockImplementation((cmd: string) =>
      cmd === 'preview_feedback' ? new Promise<IssueDraft>((r) => resolvers.push(r)) : new Promise(() => {}),
    );
    render(<FeedbackDialog onClose={vi.fn()} />);
    await userEvent.type(screen.getByRole('textbox', { name: 'Title' }), 'x');
    await waitFor(() => expect(resolvers.length).toBeGreaterThanOrEqual(2));
    const last = resolvers.length - 1;

    resolvers[last]?.({ ...DRAFT, title: 'newer' });
    await screen.findByText(/newer/);
    resolvers[0]?.({ ...DRAFT, title: 'stale' });

    await waitFor(() => expect(screen.getByLabelText('Issue preview').textContent).toContain('newer'));
    expect(screen.queryByText(/stale/)).toBeNull();
  });
});
