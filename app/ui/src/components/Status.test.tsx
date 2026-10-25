import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Callout, SegmentedControl, StatusPill, TypeTag, type OutputKind } from './Status';

describe('TypeTag', () => {
  it.each<[OutputKind, string]>([
    ['app', 'App'],
    ['image', 'Image'],
    ['deck', 'Deck'],
    ['document', 'Document'],
    ['motion', 'Motion'],
    ['diagram', 'Diagram'],
    ['architecture', 'Architecture'],
    ['data', 'Data model'],
  ])('TypeTag_Kind_%s_ShowsLabelInItsTagColors', (kind, label) => {
    render(<TypeTag kind={kind} />);

    const tag = screen.getByText(label);
    expect(tag).toHaveStyle({ background: `var(--tag-${kind})`, color: `var(--tag-${kind}-text)` });
  });
});

describe('StatusPill', () => {
  it('StatusPill_IconGiven_ShowsIconInsteadOfDot', () => {
    const { container } = render(
      <StatusPill tone="policy" icon="shield">
        Local only
      </StatusPill>,
    );

    expect(screen.getByText('Local only')).toHaveClass('c-pill', 'c-pill--policy');
    expect(container.querySelector('svg')).not.toBeNull();
    expect(container.querySelector('.c-dot')).toBeNull();
  });

  it('StatusPill_NoIconOrTone_ShowsNeutralDot', () => {
    const { container } = render(<StatusPill>Idle</StatusPill>);

    expect(screen.getByText('Idle')).toHaveClass('c-pill--neutral');
    expect(container.querySelector('.c-dot')).not.toBeNull();
    expect(container.querySelector('svg')).toBeNull();
  });
});

describe('Callout', () => {
  it('Callout_DangerTone_IsAnnouncedAsAlert', () => {
    render(<Callout tone="danger" title="Copilot is paused" />);

    expect(screen.getByRole('alert')).toHaveTextContent('Copilot is paused');
  });

  it.each(['copilot', 'policy', 'neutral'] as const)('Callout_Tone_%s_IsNotAnAlert', (tone) => {
    render(<Callout tone={tone} title="Note" />);

    expect(screen.queryByRole('alert')).toBeNull();
  });

  it('Callout_AllSlotsGiven_RendersTitleBodyAndActions', () => {
    const { container } = render(
      <Callout tone="policy" title="Rule: tools.network = off" actions={<button type="button">Request exception</button>}>
        Copilot can't reach the network in this project.
      </Callout>,
    );

    expect(container.querySelector('.c-callout__title')).toHaveTextContent('Rule: tools.network = off');
    expect(container.querySelector('.c-callout__body')).toHaveTextContent("Copilot can't reach the network in this project.");
    expect(screen.getByRole('button', { name: 'Request exception' })).toBeInTheDocument();
    expect(container.firstElementChild).toHaveClass('c-callout--policy');
  });

  it('Callout_OnlyBodyGiven_OmitsTitleAndActions', () => {
    const { container } = render(<Callout>Saved locally.</Callout>);

    expect(container.querySelector('.c-callout__title')).toBeNull();
    expect(container.querySelector('.c-callout__actions')).toBeNull();
    expect(container.firstElementChild).toHaveClass('c-callout--neutral');
  });
});

describe('SegmentedControl', () => {
  const options = [
    { value: 'dark', label: 'Dark' },
    { value: 'light', label: 'Light' },
  ];

  it('SegmentedControl_Value_ChecksOnlyMatchingRadio', () => {
    render(<SegmentedControl label="Theme" options={options} value="light" onChange={() => {}} />);

    expect(screen.getByRole('radiogroup', { name: 'Theme' })).toBeInTheDocument();
    expect(screen.getByRole('radio', { name: 'Light' })).toHaveAttribute('aria-checked', 'true');
    expect(screen.getByRole('radio', { name: 'Dark' })).toHaveAttribute('aria-checked', 'false');
  });

  it('SegmentedControl_OptionClicked_CallsOnChangeWithItsValue', async () => {
    const onChange = vi.fn();
    render(<SegmentedControl label="Theme" options={options} value="dark" onChange={onChange} />);

    await userEvent.click(screen.getByRole('radio', { name: 'Light' }));

    expect(onChange).toHaveBeenCalledExactlyOnceWith('light');
  });
});
