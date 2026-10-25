import { Button } from '../components/Button';
import { Icon, Mark } from '../components/Icon';
import { SegmentedControl, StatusPill, TypeTag, type OutputKind } from '../components/Status';
import './start.css';

const TYPES: { kind: OutputKind; blurb: string; formats: string }[] = [
  { kind: 'app', blurb: 'Screens, flows and clickable prototypes that ship as code.', formats: 'React · spec · PR' },
  { kind: 'image', blurb: 'Banners, social posts, ads, illustrations and icons.', formats: 'SVG · PNG · every size' },
  { kind: 'deck', blurb: 'Presentations with speaker notes, built on your slide masters.', formats: '.pptx · PDF · present' },
  { kind: 'document', blurb: 'One-pagers, reports, briefs and print layouts.', formats: 'PDF · .docx · print' },
  { kind: 'motion', blurb: 'Short animations, product loops and animated UI states.', formats: 'MP4 · GIF · Lottie' },
  { kind: 'diagram', blurb: 'Process flows, swimlanes and org charts. Opens Visio files.', formats: '.vsdx · SVG · Mermaid' },
  { kind: 'architecture', blurb: 'Azure services with real configs, drawn with Azure’s own icons.', formats: 'Bicep · Terraform · PR' },
  { kind: 'data', blurb: 'Entities, relationships and classifications that generate DDL.', formats: 'DDL · migrations · PR' },
];

export type Theme = 'dark' | 'light' | 'system';

export function Start({ theme, onTheme, onFeedback }: { theme: Theme; onTheme: (t: Theme) => void; onFeedback: () => void }) {
  return (
    <div className="start">
      <header className="start-bar">
        <Mark />
        <strong className="start-name">CADE</strong>
        <span className="start-sub">Copilot ADE</span>
        <span className="start-grow" />
        <StatusPill tone="policy" icon="shield">Local only</StatusPill>
        <SegmentedControl label="Theme" value={theme} onChange={onTheme} options={[{ value: 'dark', label: 'Dark' }, { value: 'light', label: 'Light' }, { value: 'system', label: 'System' }]} />
        <Button icon="comment" onClick={onFeedback}>Send feedback</Button>
      </header>
      <main className="start-main">
        <h1 className="t-page-title">What are you making?</h1>
        <div className="start-prompt">
          <label htmlFor="start-q" className="sr-only">Describe what you want to make</label>
          <input id="start-q" placeholder="Describe it — Copilot will start from your design system" />
          <button className="start-send" aria-label="Start"><Icon name="send" size={15} /></button>
        </div>
        <div className="start-grid">
          {TYPES.map((t) => (
            <button
              key={t.kind}
              className="start-card"
              aria-labelledby={`start-${t.kind}-tag`}
              aria-describedby={`start-${t.kind}-blurb start-${t.kind}-formats`}
            >
              <TypeTag kind={t.kind} id={`start-${t.kind}-tag`} />
              <span id={`start-${t.kind}-blurb`} className="start-blurb">{t.blurb}</span>
              <span id={`start-${t.kind}-formats`} className="start-formats">{t.formats}</span>
            </button>
          ))}
        </div>
      </main>
    </div>
  );
}
