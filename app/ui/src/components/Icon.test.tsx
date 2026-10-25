import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Icon, Mark } from './Icon';

describe('Icon', () => {
  it('Icon_NoLabel_IsHiddenFromAssistiveTech', () => {
    const { container } = render(<Icon name="sparkle" />);

    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('aria-hidden', 'true');
    expect(svg).not.toHaveAttribute('role');
    expect(screen.queryByRole('img')).toBeNull();
  });

  it('Icon_WithLabel_IsNamedImage', () => {
    render(<Icon name="shield" label="Organization rule" />);

    const img = screen.getByRole('img', { name: 'Organization rule' });
    expect(img).not.toHaveAttribute('aria-hidden');
  });

  it('Icon_SizeGiven_SetsWidthAndHeight', () => {
    const { container } = render(<Icon name="check" size={28} />);

    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('width', '28');
    expect(svg).toHaveAttribute('height', '28');
  });

  it('Icon_NoSize_DefaultsTo16OnA24Grid', () => {
    const { container } = render(<Icon name="close" />);

    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('width', '16');
    expect(svg).toHaveAttribute('viewBox', '0 0 24 24');
    expect(svg).toHaveAttribute('stroke', 'currentColor');
    expect(svg).toHaveAttribute('stroke-width', '1.8');
  });
});

describe('Mark', () => {
  it('Mark_Rendered_IsDecorativeAt26px', () => {
    const { container } = render(<Mark />);

    const img = container.querySelector('img');
    expect(img).toHaveAttribute('alt', '');
    expect(img).toHaveAttribute('aria-hidden', 'true');
    expect(img).toHaveAttribute('width', '26');
    expect(img).toHaveAttribute('height', '26');
  });

  it('Mark_Rendered_ShowsTheSparkleAppIcon', () => {
    const { container } = render(<Mark size={32} />);

    expect(container.querySelector('img')?.getAttribute('src')).toMatch(/cade-mark\.png$/);
    expect(container.querySelector('svg')).toBeNull();
  });
});
