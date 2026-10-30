import { useEffect, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { Button } from '../components/Button';
import { Icon } from '../components/Icon';
import { call, inTauri } from '../ipc';
import { version } from '../../package.json';
import { FORMS as METADATA } from './forms.generated';
import { FORMS, type Feedback, type FeedbackKind, type IssueDraft } from './types';
import './feedback.css';

const KINDS: FeedbackKind[] = ['bug', 'feature-request', 'comment'];
const blankAnswers = (kind: FeedbackKind): Feedback['answers'] =>
  Object.fromEntries(FORMS[kind].questions.map((question) => [question.id, '']));
const FOCUSABLE = 'button:not([disabled]), input:not([disabled]), textarea:not([disabled]), select:not([disabled]), a[href], [tabindex]:not([tabindex="-1"])';

/** Modal focus: move focus in on open, keep Tab inside, close on Escape, and give focus back to the opener. */
function useModalFocus(onClose: () => void) {
  const root = useRef<HTMLElement>(null);

  useEffect(() => {
    const opener = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    const dialog = root.current;
    (dialog?.querySelector<HTMLElement>(FOCUSABLE) ?? dialog)?.focus();
    return () => opener?.focus();
  }, []);

  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      const dialog = root.current;
      if (!dialog) return;
      if (e.key === 'Escape') {
        e.preventDefault();
        onClose();
        return;
      }
      if (e.key !== 'Tab') return;
      const items = Array.from(dialog.querySelectorAll<HTMLElement>(FOCUSABLE));
      const first = items[0];
      const last = items[items.length - 1];
      const active = document.activeElement;
      if (!first || !last) {
        e.preventDefault();
        dialog.focus();
      } else if (!dialog.contains(active)) {
        e.preventDefault();
        (e.shiftKey ? last : first).focus();
      } else if (e.shiftKey && active === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && active === last) {
        e.preventDefault();
        first.focus();
      }
    }
    document.addEventListener('keydown', onKeyDown);
    return () => document.removeEventListener('keydown', onKeyDown);
  }, [onClose]);

  return root;
}

/** Associate each Rust draft with the exact input snapshot that produced it. */
function useFeedbackPreview(feedback: Feedback) {
  const key = JSON.stringify(feedback);
  const [attempt, setAttempt] = useState(0);
  const [result, setResult] = useState<
    | { key: string; attempt: number; status: 'pending' | 'failed' }
    | { key: string; attempt: number; status: 'ready'; draft: IssueDraft; snapshot: Feedback }
  >();

  useEffect(() => {
    if (!inTauri) return;
    let latest = true;
    const snapshot = JSON.parse(key) as Feedback;
    setResult({ key, attempt, status: 'pending' });
    call('preview_feedback', { feedback: snapshot })
      .then((draft) => {
        if (latest) setResult({ key, attempt, status: 'ready', draft, snapshot });
      })
      .catch(() => {
        if (latest) setResult({ key, attempt, status: 'failed' });
      });
    return () => {
      latest = false;
    };
  }, [key, attempt]);

  // Content, not object identity, determines validity even before the next effect runs.
  const current = result?.key === key && result.attempt === attempt ? result : undefined;
  return { current, retry: () => setAttempt((value) => value + 1) };
}

function useSuccessFocus(sent: boolean) {
  const heading = useRef<HTMLHeadingElement>(null);
  useLayoutEffect(() => {
    if (sent) heading.current?.focus();
  }, [sent]);
  return heading;
}

function QuestionControl({ question, value, disabled, onChange }: {
  question: (typeof FORMS)[FeedbackKind]['questions'][number];
  value: string;
  disabled: boolean;
  onChange: (value: string) => void;
}) {
  const props = { id: `fb-${question.id}`, name: question.id, required: question.required, disabled };
  return (
    <div className="c-field">
      <label htmlFor={props.id}>{question.heading} <span className="fb-muted">{question.required ? 'required' : 'optional'}</span></label>
      {question.type === 'dropdown' ? (
        <select
          {...props}
          multiple={question.multiple}
          value={question.multiple ? (value ? value.split(', ') : []) : value}
          onChange={(event) => onChange(
            question.multiple
              ? Array.from(event.currentTarget.selectedOptions, (option) => option.value).join(', ')
              : event.currentTarget.value,
          )}
        >
          {!question.multiple && <option value="">Not specified</option>}
          {question.options?.map((option) => <option key={option} value={option}>{option}</option>)}
        </select>
      ) : question.type === 'textarea' ? (
        <textarea {...props} value={value} onChange={(event) => onChange(event.currentTarget.value)} />
      ) : (
        <input {...props} value={value} onChange={(event) => onChange(event.currentTarget.value)} />
      )}
    </div>
  );
}

/** Help › Send feedback. Builds the issue in Rust (redacted), previews it, then submits as the user. */
export function FeedbackDialog({ onClose }: { onClose: () => void }) {
  const [kind, setKind] = useState<FeedbackKind>('bug');
  const [title, setTitle] = useState('');
  // One blank answer per question from the first render, so text typed before any effect runs is kept.
  const [answers, setAnswers] = useState(() => blankAnswers('bug'));
  const [env, setEnv] = useState({ app_version: true, os: true, gpu: false, policy_version: false });
  const [screenshotUrl, setScreenshotUrl] = useState('');
  const [status, setStatus] = useState<{ kind: 'idle' | 'sending' | 'sent' | 'error'; text?: string }>({ kind: 'idle' });
  const dialogRef = useModalFocus(onClose);
  const successHeading = useSuccessFocus(status.kind === 'sent');

  const form = FORMS[kind];
  const sending = status.kind === 'sending';

  function chooseKind(next: FeedbackKind) {
    if (next === kind) return;
    setKind(next);
    setAnswers(blankAnswers(next));
  }

  const feedback: Feedback = useMemo(
    () => ({
      kind,
      title,
      answers,
      environment: {
        app_version: env.app_version ? version : null,
        os: env.os ? navigator.userAgent : null,
        gpu: null,
        policy_version: null,
        logs: null,
      },
      screenshot_url: screenshotUrl || null,
    }),
    [kind, title, answers, env, screenshotUrl],
  );

  const { current, retry } = useFeedbackPreview(feedback);
  const preview = current?.status === 'ready' ? current.draft : undefined;

  const uiHasRequiredFields = title.trim() !== '' &&
    form.questions.every((q) => !q.required || (answers[q.id] ?? '').trim() !== '');
  const canSend = inTauri && current?.status === 'ready' && uiHasRequiredFields;

  async function send() {
    if (!canSend || current?.status !== 'ready' || status.kind === 'sending') return;
    const snapshot = current.snapshot;
    setStatus({ kind: 'sending' });
    try {
      const url = await call('submit_feedback', { feedback: snapshot });
      setStatus({ kind: 'sent', text: url });
    } catch (e) {
      setStatus({ kind: 'error', text: e instanceof Error ? e.message : typeof e === 'string' ? e : undefined });
    }
  }

  return (
    <div className="fb-scrim">
      <section ref={dialogRef} role="dialog" aria-modal="true" aria-labelledby="fb-title" className="fb" tabIndex={-1}>
        <header className="fb-head">
          <h1 id="fb-title">Send feedback</h1>
          <span className="fb-muted">Creates a GitHub issue from your account</span>
          <span className="fb-grow" />
          <button className="fb-close" aria-label="Close" onClick={onClose}><Icon name="close" size={14} /></button>
        </header>

        {status.kind === 'sent' ? (
          <div className="fb-done" role="status">
            <Icon name="check" size={28} />
            <h2 ref={successHeading} className="t-title" tabIndex={-1}>Issue opened</h2>
            <a href={status.text} target="_blank" rel="noreferrer">{status.text}</a>
            <Button onClick={onClose}>Close</Button>
          </div>
        ) : (
          <>
            <div className="fb-body">
              <div className="fb-form">
                <div className="fb-kinds" role="radiogroup" aria-label="Feedback type">
                  {KINDS.map((k) => (
                    <button key={k} role="radio" aria-checked={k === kind} className="fb-kind" disabled={sending} onClick={() => chooseKind(k)}>
                      <span className="t-title">{FORMS[k].name}</span>
                      <span className="fb-muted">{FORMS[k].hint}</span>
                    </button>
                  ))}
                </div>
                <div className="c-field">
                  <label htmlFor="fb-t">Title</label>
                  <input id="fb-t" value={title} disabled={sending} onChange={(e) => setTitle(e.target.value)} />
                </div>
                {form.questions.map((question) => (
                  <QuestionControl
                    key={question.id}
                    question={question}
                    value={answers[question.id] ?? ''}
                    disabled={sending}
                    onChange={(value) => setAnswers((previous) => ({ ...previous, [question.id]: value }))}
                  />
                ))}
                {METADATA.bug.body.flatMap((field) => 'id' in field && field.id === 'screenshot_url' ? [
                  <div className="c-field" key={field.id}>
                    <label htmlFor="fb-screenshot">{field.attributes.label} <span className="fb-muted">optional</span></label>
                    <input id="fb-screenshot" name={field.id} value={screenshotUrl} disabled={sending} onChange={(event) => setScreenshotUrl(event.currentTarget.value)} />
                  </div>,
                ] : [])}
              </div>
              <aside className="fb-side" aria-label="What GitHub will receive">
                <span className="t-section-label fb-muted">WHAT GITHUB WILL RECEIVE</span>
                {METADATA.bug.body.flatMap((field) => {
                  if (!('id' in field) || !(field.id === 'app_version' || field.id === 'os' || field.id === 'gpu' || field.id === 'policy_version')) return [];
                  const key = field.id;
                  const unavailable = key === 'gpu' || key === 'policy_version';
                  return [
                    <div className="fb-detail" key={key}>
                      <label className="fb-check">
                        <input
                          type="checkbox"
                          name={key}
                          checked={env[key]}
                          disabled={sending || unavailable}
                          aria-describedby={unavailable ? `fb-${key}-unavailable` : undefined}
                          onChange={() => setEnv((previous) => ({ ...previous, [key]: !previous[key] }))}
                        />
                        {field.attributes.label}
                      </label>
                      {unavailable && <span id={`fb-${key}-unavailable`} className="fb-muted fb-check-note">Not available. This detail will not be sent.</span>}
                    </div>,
                  ];
                })}
                {/* CADE keeps no local logs yet (FOUND-014), so nothing can be attached; enable this once it does. */}
                <label className="fb-check">
                  <input type="checkbox" checked={false} disabled aria-describedby="fb-logs-unavailable" />
                  Recent logs, scrubbed
                </label>
                <span id="fb-logs-unavailable" className="fb-muted fb-check-note">Recent logs are not available. No logs will be sent.</span>
                <div className="fb-safe">Never sent unless you attach it: designs, prompts, Copilot threads, file paths and project names.</div>
                {inTauri && !uiHasRequiredFields && (
                  <span className="fb-muted">Complete the required fields to preview your issue.</span>
                )}
                {inTauri && uiHasRequiredFields && (!current || current.status === 'pending') && (
                  <span className="fb-muted" role="status">Updating issue preview…</span>
                )}
                {inTauri && uiHasRequiredFields && current?.status === 'failed' && (
                  <>
                    <span className="fb-error" role="alert">Couldn’t preview your issue. Your entries are still in this form.</span>
                    <Button onClick={retry}>Retry preview</Button>
                  </>
                )}
                {preview && (
                  <>
                    <span className="fb-muted">Labels</span>
                    <span aria-label="Labels">{preview.labels.join(', ')}</span>
                    <pre className="fb-preview" aria-label="Issue preview">{preview.title}{'\n\n'}{preview.body}</pre>
                  </>
                )}
              </aside>
            </div>
            <footer className="fb-foot">
              {status.kind === 'error' && (
                <span className="fb-error" role="alert">
                  <span>Couldn’t submit your issue. Your entries are still in this form.</span>
                  {status.text && <> {status.text}</>}
                </span>
              )}
              {!inTauri && <span className="fb-muted">Open the desktop app to send.</span>}
              <span className="fb-grow" />
              <Button onClick={onClose}>Cancel</Button>
              <Button variant="primary" disabled={!canSend || status.kind === 'sending' || !inTauri} onClick={send}>{status.kind === 'sending' ? 'Sending…' : form.cta}</Button>
            </footer>
          </>
        )}
      </section>
    </div>
  );
}
