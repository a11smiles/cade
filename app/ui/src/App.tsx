import { useEffect, useState } from 'react';
import { FeedbackDialog } from './feedback/FeedbackDialog';
import { Start, type Theme } from './screens/Start';

function savedTheme(): Theme {
  try {
    const t = localStorage.getItem('cade.theme');
    return t === 'light' || t === 'system' ? t : 'dark';
  } catch {
    return 'dark';
  }
}

export function App() {
  const [theme, setTheme] = useState<Theme>(savedTheme);
  const [feedback, setFeedback] = useState(false);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    try {
      localStorage.setItem('cade.theme', theme);
    } catch {
      /* storage unavailable */
    }
  }, [theme]);

  return (
    <>
      <Start theme={theme} onTheme={setTheme} onFeedback={() => setFeedback(true)} />
      {feedback && <FeedbackDialog onClose={() => setFeedback(false)} />}
    </>
  );
}
