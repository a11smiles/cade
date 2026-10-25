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
});
