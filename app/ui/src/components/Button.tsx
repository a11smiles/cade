import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { Icon, type IconName } from './Icon';
import './components.css';

type Variant = 'primary' | 'secondary' | 'quiet' | 'copilot' | 'policy' | 'success' | 'danger';

export function Button({ variant = 'secondary', size = 'md', icon, children, ...rest }: { variant?: Variant; size?: 'sm' | 'md' | 'lg'; icon?: IconName; children?: ReactNode } & ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button type="button" className={`c-btn c-btn--${variant} c-btn--${size}`} {...rest}>
      {icon && <Icon name={icon} size={size === 'sm' ? 12 : 14} />}
      {children}
    </button>
  );
}
