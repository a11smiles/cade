import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Start } from './Start';

function renderStart(overrides: Partial<Parameters<typeof Start>[0]> = {}) {
  const props = { theme: 'dark' as const, onTheme: vi.fn(), onFeedback: vi.fn(), ...overrides };
  render(<Start {...props} />);
  return props;
}

describe('Start', () => {
  it('Start_Rendered_AsksWhatTheUserIsMaking', () => {
    renderStart();

    expect(screen.getByRole('heading', { level: 1, name: 'What are you making?' })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: 'Describe what you want to make' })).toHaveAttribute(
      'placeholder',
      'Describe it — Copilot will start from your design system',
    );
    expect(screen.getByRole('button', { name: 'Start' })).toBeInTheDocument();
  });

  it.each([
    ['App', 'React · spec · PR'],
    ['Image', 'SVG · PNG · every size'],
    ['Deck', '.pptx · PDF · present'],
    ['Document', 'PDF · .docx · print'],
    ['Motion', 'MP4 · GIF · Lottie'],
    ['Diagram', '.vsdx · SVG · Mermaid'],
    ['Architecture', 'Bicep · Terraform · PR'],
    ['Data model', 'DDL · migrations · PR'],
  ])('Start_OutputCard_%s_ShowsItsFormats', (label, formats) => {
    renderStart();

    const card = screen.getByRole('button', { name: label });
    expect(within(card).getByText(formats)).toBeInTheDocument();
  });

  it.each([
    ['App', 'Screens, flows and clickable prototypes that ship as code.', 'React · spec · PR'],
    ['Architecture', 'Azure services with real configs, drawn with Azure’s own icons.', 'Bicep · Terraform · PR'],
    ['Data model', 'Entities, relationships and classifications that generate DDL.', 'DDL · migrations · PR'],
  ])('Start_OutputCard_%s_IsNamedByTypeAndDescribedByBlurbAndFormats', (label, blurb, formats) => {
    renderStart();

    expect(screen.getByRole('button', { name: label })).toHaveAccessibleDescription(`${blurb} ${formats}`);
  });

  it('Start_Rendered_ShowsExactlyEightOutputCards', () => {
    const { container } = render(<Start theme="dark" onTheme={vi.fn()} onFeedback={vi.fn()} />);

    expect(container.querySelectorAll('.start-card')).toHaveLength(8);
  });

  it('Start_Rendered_ShowsLocalOnlyGovernancePill', () => {
    renderStart();

    expect(screen.getByText('Local only')).toHaveClass('c-pill--policy');
  });

  it('Start_ThemeProp_ChecksMatchingThemeOption', () => {
    renderStart({ theme: 'system' });

    const group = screen.getByRole('radiogroup', { name: 'Theme' });
    expect(within(group).getByRole('radio', { name: 'System' })).toHaveAttribute('aria-checked', 'true');
    expect(within(group).getByRole('radio', { name: 'Dark' })).toHaveAttribute('aria-checked', 'false');
  });

  it('Start_ThemeOptionClicked_CallsOnTheme', async () => {
    const props = renderStart();

    await userEvent.click(screen.getByRole('radio', { name: 'Light' }));

    expect(props.onTheme).toHaveBeenCalledExactlyOnceWith('light');
  });

  it('Start_SendFeedbackClicked_CallsOnFeedback', async () => {
    const props = renderStart();

    await userEvent.click(screen.getByRole('button', { name: 'Send feedback' }));

    expect(props.onFeedback).toHaveBeenCalledOnce();
  });
});
