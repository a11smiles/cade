import { render, screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { FeedbackDialog } from './FeedbackDialog';
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

  it.each([
    [/^Feature request/, 'Submit request', ['What problem would this solve?', 'What would you like CADE to do?', 'Which output type?']],
    [/^Comment/, 'Send comment', ['Your comment', 'Where in CADE?']],
  ])('FeedbackDialog_KindChosen_%s_ShowsThatFormsQuestions', async (kind, cta, headings) => {
    render(<FeedbackDialog onClose={vi.fn()} />);

    await userEvent.click(screen.getByRole('radio', { name: kind }));

    expect(screen.getByRole('radio', { name: kind })).toHaveAttribute('aria-checked', 'true');
    expect(screen.getAllByRole('textbox').slice(1).map((t) => t.id)).toHaveLength(headings.length);
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
    expect(fb.answers).toEqual([
      { heading: 'What happened?', text: 'It froze.' },
      { heading: 'Steps to reproduce', text: 'Open a big file.' },
      { heading: 'What did you expect?', text: '' },
    ]);
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
    await waitFor(() => expect(lastPreview().environment[key]).not.toBeNull());

    await userEvent.click(screen.getByRole('checkbox', { name: label }));

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
