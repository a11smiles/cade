import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { Button } from './Button';

describe('Button', () => {
  it('Button_NoTypeGiven_DoesNotSubmitForms', () => {
    render(<Button>Apply 6 changes</Button>);

    expect(screen.getByRole('button', { name: 'Apply 6 changes' })).toHaveAttribute('type', 'button');
  });

  it('Button_KeyboardActivation_CallsOnClick', async () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Open pull request</Button>);

    await userEvent.tab();
    await userEvent.keyboard('{Enter}');

    expect(screen.getByRole('button', { name: 'Open pull request' })).toHaveFocus();
    expect(onClick).toHaveBeenCalledOnce();
  });

  it('Button_Disabled_IgnoresClicks', async () => {
    const onClick = vi.fn();
    render(
      <Button disabled onClick={onClick}>
        Submit bug
      </Button>,
    );

    await userEvent.click(screen.getByRole('button', { name: 'Submit bug' }));

    expect(screen.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
    expect(onClick).not.toHaveBeenCalled();
  });

  it('Button_VariantAndSize_AppliesDesignSystemClasses', () => {
    render(
      <Button variant="primary" size="lg">
        Ship
      </Button>,
    );

    expect(screen.getByRole('button', { name: 'Ship' })).toHaveClass('c-btn', 'c-btn--primary', 'c-btn--lg');
  });

  it('Button_NoVariantOrSize_DefaultsToSecondaryMedium', () => {
    render(<Button>Cancel</Button>);

    expect(screen.getByRole('button', { name: 'Cancel' })).toHaveClass('c-btn--secondary', 'c-btn--md');
  });

  it('Button_IconGiven_RendersDecorativeIconSizedToButton', () => {
    render(
      <Button icon="comment" size="sm">
        Send feedback
      </Button>,
    );

    const svg = screen.getByRole('button', { name: 'Send feedback' }).querySelector('svg');
    expect(svg).toHaveAttribute('aria-hidden', 'true');
    expect(svg).toHaveAttribute('width', '12');
  });
});
