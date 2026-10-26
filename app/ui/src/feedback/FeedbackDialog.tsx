import { useEffect, useMemo, useState } from 'react';
import { Button } from '../components/Button';
import { Icon } from '../components/Icon';
import { call, inTauri } from '../ipc';
import { FORMS, type Feedback, type FeedbackKind, type IssueDraft } from './types';
import './feedback.css';

const KINDS: FeedbackKind[] = ['bug', 'feature-request', 'comment'];

/** Help › Send feedback. Builds the issue in Rust (redacted), previews it, then submits as the user. */
export function FeedbackDialog({ onClose }: { onClose: () => void }) {
  const [kind, setKind] = useState<FeedbackKind>('bug');
  const [title, setTitle] = useState('');
  const [answers, setAnswers] = useState<string[]>([]);
  const [env, setEnv] = useState({ version: true, os: true, gpu: true, policy: true, logs: false });
  const [preview, setPreview] = useState<IssueDraft | null>(null);
  const [status, setStatus] = useState<{ kind: 'idle' | 'sending' | 'sent' | 'error'; text?: string }>({ kind: 'idle' });

  const form = FORMS[kind];
  useEffect(() => setAnswers(form.questions.map(() => '')), [kind]); // eslint-disable-line react-hooks/exhaustive-deps

  const feedback: Feedback = useMemo(
    () => ({
      kind,
      title,
      answers: form.questions.map((q, i) => ({ heading: q.heading, text: answers[i] ?? '' })),
      environment: {
        app_version: env.version ? '0.1.0' : null,
        os: env.os ? navigator.userAgent : null,
        gpu: env.gpu ? 'reported by cade-render' : null,
        policy_version: env.policy ? 'from current policy' : null,
        logs: null,
      },
    }),
    [kind, title, answers, env, form],
  );

  const feedbackKey = JSON.stringify(feedback);
  useEffect(() => {
    if (!inTauri) return;
    let current = true;
    call<IssueDraft>('preview_feedback', { feedback: JSON.parse(feedbackKey) as Feedback })
      .then((draft) => current && setPreview(draft))
      .catch(() => current && setPreview(null));
    return () => {
      current = false;
    };
  }, [feedbackKey]);

  const canSend = title.trim() !== '' && form.questions.every((q, i) => !q.required || (answers[i] ?? '').trim() !== '');

  async function send() {
    setStatus({ kind: 'sending' });
    try {
      const url = await call<string>('submit_feedback', { feedback });
      setStatus({ kind: 'sent', text: url });
    } catch (e) {
      setStatus({ kind: 'error', text: String(e) });
    }
  }

  return (
    <div className="fb-scrim">
      <section role="dialog" aria-modal="true" aria-labelledby="fb-title" className="fb">
        <header className="fb-head">
          <h1 id="fb-title">Send feedback</h1>
          <span className="fb-muted">Creates a GitHub issue from your account</span>
          <span className="fb-grow" />
          <button className="fb-close" aria-label="Close" onClick={onClose}><Icon name="close" size={14} /></button>
        </header>

        {status.kind === 'sent' ? (
          <div className="fb-done" role="status">
            <Icon name="check" size={28} />
            <p className="t-title">Issue opened</p>
            <a href={status.text} target="_blank" rel="noreferrer">{status.text}</a>
            <Button onClick={onClose}>Close</Button>
          </div>
        ) : (
          <>
            <div className="fb-body">
              <div className="fb-form">
                <div className="fb-kinds" role="radiogroup" aria-label="Feedback type">
                  {KINDS.map((k) => (
                    <button key={k} role="radio" aria-checked={k === kind} className="fb-kind" onClick={() => setKind(k)}>
                      <span className="t-title">{FORMS[k].name}</span>
                      <span className="fb-muted">{FORMS[k].hint}</span>
                    </button>
                  ))}
                </div>
                <div className="c-field">
                  <label htmlFor="fb-t">Title</label>
                  <input id="fb-t" value={title} onChange={(e) => setTitle(e.target.value)} />
                </div>
                {form.questions.map((q, i) => (
                  <div className="c-field" key={q.heading}>
                    <label htmlFor={`fb-q${i}`}>{q.heading} <span className="fb-muted">{q.required ? 'required' : 'optional'}</span></label>
                    <textarea id={`fb-q${i}`} value={answers[i] ?? ''} onChange={(e) => setAnswers((a) => a.map((x, j) => (j === i ? e.target.value : x)))} />
                  </div>
                ))}
              </div>
              <aside className="fb-side" aria-label="What GitHub will receive">
                <span className="t-section-label fb-muted">WHAT GITHUB WILL RECEIVE</span>
                {(['version', 'os', 'gpu', 'policy', 'logs'] as const).map((k) => (
                  <label key={k} className="fb-check">
                    <input type="checkbox" checked={env[k]} onChange={() => setEnv((e) => ({ ...e, [k]: !e[k] }))} />
                    {{ version: 'CADE version', os: 'Operating system', gpu: 'GPU and renderer', policy: 'Policy version', logs: 'Recent logs, scrubbed' }[k]}
                  </label>
                ))}
                <div className="fb-safe">Never sent unless you attach it: designs, prompts, Copilot threads, file paths and project names.</div>
                {preview && <pre className="fb-preview" aria-label="Issue preview">{preview.title}{'\n\n'}{preview.body}</pre>}
              </aside>
            </div>
            <footer className="fb-foot">
              {status.kind === 'error' && <span className="fb-error" role="alert">{status.text}</span>}
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
