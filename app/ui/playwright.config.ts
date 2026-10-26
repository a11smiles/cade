import { defineConfig, devices } from '@playwright/test';

const PORT = 4173;
const ci = !!process.env.CI;

// E2E runs the UI in Chromium against the Vite dev server. Specs either run it
// as a plain browser (outside Tauri) or inject a mock Tauri IPC bridge
// (e2e/tauri.ts); the native shell itself is covered by `cargo test -p cade-app`.
export default defineConfig({
  testDir: 'e2e',
  fullyParallel: true,
  forbidOnly: ci,
  retries: 0,
  reporter: ci ? [['list'], ['html', { open: 'never' }]] : 'list',
  use: {
    baseURL: `http://localhost:${PORT}`,
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'default-window', use: { ...devices['Desktop Chrome'], viewport: { width: 1440, height: 900 } } },
    { name: 'min-window', use: { ...devices['Desktop Chrome'], viewport: { width: 1100, height: 700 } } },
  ],
  webServer: {
    command: `pnpm tokens && pnpm exec vite --port ${PORT} --strictPort`,
    url: `http://localhost:${PORT}`,
    reuseExistingServer: !ci,
    timeout: 60_000,
  },
});
