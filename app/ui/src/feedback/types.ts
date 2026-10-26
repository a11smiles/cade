import type { FeedbackKind } from '../ipc/bindings';
import { FORMS as METADATA } from './forms.generated';

export type { Answer, Environment, Feedback, FeedbackKind, IssueDraft } from '../ipc/bindings';

/** Display adaptation only: YAML owns field IDs, types, labels, options and requiredness. */
function displayForm(kind: FeedbackKind, cta: string) {
  const metadata = METADATA[kind];
  return {
    name: metadata.name,
    hint: metadata.description,
    cta,
    questions: metadata.body.flatMap((field) => {
      if (!('id' in field) || ['app_version', 'os', 'gpu', 'policy_version', 'screenshot_url'].includes(field.id)) return [];
      return [{
        id: field.id,
        heading: field.attributes.label,
        type: field.type,
        required: 'validations' in field && field.validations.required,
        multiple: 'multiple' in field.attributes && field.attributes.multiple,
        options: 'options' in field.attributes ? field.attributes.options : undefined,
      }];
    }),
  };
}

// Preserve the existing UI facade and action copy without mirroring the IPC contract.
export const FORMS = {
  bug: displayForm('bug', 'Submit bug'),
  'feature-request': displayForm('feature-request', 'Submit request'),
  comment: displayForm('comment', 'Send comment'),
};
