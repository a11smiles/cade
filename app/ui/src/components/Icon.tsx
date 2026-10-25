const PATHS = {
  sparkle: 'M12 3l1.8 5.2L19 10l-5.2 1.8L12 17l-1.8-5.2L5 10l5.2-1.8z',
  shield: 'M12 3l8 3v6c0 5-3.5 8-8 9-4.5-1-8-4-8-9V6z',
  send: 'M12 19V5M5 12l7-7 7 7',
  check: 'M5 12l5 5 9-10',
  warning: 'M12 4l9 16H3zM12 10v4M12 17v.5',
  import: 'M12 15V3M7 8l5-5 5 5M4 15v5h16v-5',
  comment: 'M4 5h16v11H9l-5 4z',
  close: 'M6 6l12 12M18 6L6 18',
  settings: 'M12 9a3 3 0 110 6 3 3 0 010-6zM12 2v3M12 19v3M2 12h3M19 12h3M4.9 4.9l2.1 2.1M17 17l2.1 2.1M4.9 19.1L7 17M17 7l2.1-2.1',
} as const;

export type IconName = keyof typeof PATHS;

export function Icon({ name, size = 16, label }: { name: IconName; size?: number; label?: string }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8} strokeLinecap="round" strokeLinejoin="round" role={label ? 'img' : undefined} aria-label={label} aria-hidden={label ? undefined : true}>
      <path d={PATHS[name]} />
    </svg>
  );
}

export function Mark({ size = 26 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 26 26" aria-hidden="true">
      <rect x="1" y="1" width="24" height="24" rx="7" fill="#6B57F0" />
      <path d="M8 18c2-1 3-4 5-7s4-3 5-2-1 3-3 5-5 3-7 4z" fill="none" stroke="#fff" strokeWidth="1.8" strokeLinejoin="round" />
      <circle cx="8" cy="18" r="1.6" fill="#F0B44C" />
    </svg>
  );
}
