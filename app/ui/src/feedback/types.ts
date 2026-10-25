// Mirrors cade_feedback::Feedback (Rust). Keep in sync.
export type FeedbackKind = 'bug' | 'feature-request' | 'comment';
export interface Answer { heading: string; text: string }
export interface Environment { app_version?: string | null; os?: string | null; gpu?: string | null; policy_version?: string | null; logs?: string | null }
export interface Feedback { kind: FeedbackKind; title: string; answers: Answer[]; environment: Environment; screenshot_url?: string | null }
export interface IssueDraft { title: string; body: string; labels: string[] }

export const FORMS: Record<FeedbackKind, { name: string; hint: string; cta: string; questions: { heading: string; required: boolean }[] }> = {
  bug: {
    name: 'Bug', hint: 'Something is broken or wrong.', cta: 'Submit bug',
    questions: [
      { heading: 'What happened?', required: true },
      { heading: 'Steps to reproduce', required: true },
      { heading: 'What did you expect?', required: false },
    ],
  },
  'feature-request': {
    name: 'Feature request', hint: 'Something CADE should do.', cta: 'Submit request',
    questions: [
      { heading: 'What problem would this solve?', required: true },
      { heading: 'What would you like CADE to do?', required: true },
      { heading: 'Which output type?', required: false },
    ],
  },
  comment: {
    name: 'Comment', hint: 'Anything else: praise, questions, ideas.', cta: 'Send comment',
    questions: [
      { heading: 'Your comment', required: true },
      { heading: 'Where in CADE?', required: false },
    ],
  },
};
