import type { ReactNode } from 'react';
import { Icon, type IconName } from './Icon';
import './components.css';

export type OutputKind = 'app' | 'image' | 'deck' | 'document' | 'motion' | 'diagram' | 'architecture' | 'data';
const LABEL: Record<OutputKind, string> = { app: 'App', image: 'Image', deck: 'Deck', document: 'Document', motion: 'Motion', diagram: 'Diagram', architecture: 'Architecture', data: 'Data model' };

export function TypeTag({ kind, id }: { kind: OutputKind; id?: string }) {
  return <span id={id} className="c-tag" style={{ background: `var(--tag-${kind})`, color: `var(--tag-${kind}-text)` }}>{LABEL[kind]}</span>;
}

export function StatusPill({ tone = 'neutral', icon, children }: { tone?: 'policy' | 'success' | 'danger' | 'accent' | 'neutral'; icon?: IconName; children: ReactNode }) {
  return <span className={`c-pill c-pill--${tone}`}>{icon ? <Icon name={icon} size={12} /> : <span className="c-dot" />}{children}</span>;
}

export function Callout({ tone = 'neutral', title, children, actions }: { tone?: 'copilot' | 'policy' | 'danger' | 'neutral'; title?: ReactNode; children?: ReactNode; actions?: ReactNode }) {
  const icon: IconName = tone === 'policy' ? 'shield' : tone === 'danger' ? 'warning' : tone === 'copilot' ? 'sparkle' : 'comment';
  return (
    <div className={`c-callout c-callout--${tone}`} role={tone === 'danger' ? 'alert' : undefined}>
      {title && <div className="c-callout__title"><Icon name={icon} size={14} />{title}</div>}
      {children && <div className="c-callout__body">{children}</div>}
      {actions && <div className="c-callout__actions">{actions}</div>}
    </div>
  );
}

export function SegmentedControl<T extends string>({ label, options, value, onChange }: { label: string; options: { value: T; label: string }[]; value: T; onChange: (v: T) => void }) {
  return (
    <div className="c-seg" role="radiogroup" aria-label={label}>
      {options.map((o) => (
        <button key={o.value} type="button" role="radio" aria-checked={o.value === value} onClick={() => onChange(o.value)}>{o.label}</button>
      ))}
    </div>
  );
}
