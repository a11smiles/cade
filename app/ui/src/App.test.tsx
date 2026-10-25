import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { App } from './App';

vi.mock('./ipc', () => ({ inTauri: false, call: vi.fn() }));

beforeEach(() => {
  localStorage.clear();
  delete document.documentElement.dataset.theme;
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe('App', () => {
  it('App_NothingSaved_StartsInDarkTheme', () => {
    render(<App />);

    expect(document.documentElement.dataset.theme).toBe('dark');
    expect(screen.getByRole('radio', { name: 'Dark' })).toHaveAttribute('aria-checked', 'true');
  });

  it.each(['light', 'system'])('App_SavedTheme_%s_IsRestored', (theme) => {
    localStorage.setItem('cade.theme', theme);

    render(<App />);

    expect(document.documentElement.dataset.theme).toBe(theme);
  });

  it('App_UnknownSavedTheme_FallsBackToDark', () => {
    localStorage.setItem('cade.theme', 'sepia');

    render(<App />);

    expect(document.documentElement.dataset.theme).toBe('dark');
  });

  it('App_ThemeChosen_AppliesAndSavesIt', async () => {
    render(<App />);

    await userEvent.click(screen.getByRole('radio', { name: 'Light' }));

    expect(document.documentElement.dataset.theme).toBe('light');
    expect(localStorage.getItem('cade.theme')).toBe('light');
  });

  it('App_StorageUnavailable_StillRendersInDarkTheme', async () => {
    vi.stubGlobal('localStorage', {
      getItem: () => {
        throw new Error('blocked');
      },
      setItem: () => {
        throw new Error('blocked');
      },
    });

    render(<App />);
    await userEvent.click(screen.getByRole('radio', { name: 'Light' }));

    expect(document.documentElement.dataset.theme).toBe('light');
  });

  it('App_SendFeedbackClicked_OpensAndClosesFeedbackDialog', async () => {
    render(<App />);
    expect(screen.queryByRole('dialog')).toBeNull();

    await userEvent.click(screen.getByRole('button', { name: 'Send feedback' }));
    expect(screen.getByRole('dialog', { name: 'Send feedback' })).toBeInTheDocument();

    await userEvent.click(screen.getByRole('button', { name: 'Close' }));
    expect(screen.queryByRole('dialog')).toBeNull();
  });

  it('App_FeedbackDialogEscaped_ClosesAndReturnsFocusToSendFeedback', async () => {
    render(<App />);
    await userEvent.click(screen.getByRole('button', { name: 'Send feedback' }));

    await userEvent.keyboard('{Escape}');

    expect(screen.queryByRole('dialog')).toBeNull();
    expect(screen.getByRole('button', { name: 'Send feedback' })).toHaveFocus();
  });
});
