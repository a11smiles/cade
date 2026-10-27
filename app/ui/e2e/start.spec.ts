import { expect, test } from '@playwright/test';
import { mockTauri } from './tauri';
import { tokenRgb } from './tokens';

test.describe('Start screen', () => {
  test('Start_Loaded_ShowsEveryOutputTypeWithoutErrors', async ({ browser, baseURL }, testInfo) => {
    for (const tauri of [false, true]) {
      // Each launch must obtain the actual bytes, not reuse the other mode's cache.
      const context = await browser.newContext({ baseURL, viewport: testInfo.project.use.viewport });
      try {
        const page = await context.newPage();
        const errors: string[] = [];
        const requests: string[] = [];
        const fontResponses: { url: string; status: number }[] = [];
        page.on('pageerror', (e) => errors.push(e.message));
        page.on('console', (m) => m.type() === 'error' && errors.push(m.text()));
        page.on('request', (request) => requests.push(request.url()));
        page.on('requestfailed', (request) => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
        page.on('response', (response) => {
          if (response.request().resourceType() === 'font') {
            fontResponses.push({ url: response.url(), status: response.status() });
          }
        });
        if (tauri) await mockTauri(page);
        await page.goto('/');

        await expect(page).toHaveTitle('CADE');
        await expect(page.getByRole('heading', { level: 1, name: 'What are you making?' })).toBeVisible();
        for (const label of ['App', 'Image', 'Deck', 'Document', 'Motion', 'Diagram', 'Architecture', 'Data model']) {
          await expect(page.getByRole('button', { name: label, exact: true })).toBeVisible();
        }
        await expect(page.getByText('Local only')).toBeVisible();

        for (const theme of ['Dark', 'Light']) {
          await page.getByRole('radio', { name: theme, exact: true }).click();
          const loaded = await page.evaluate(async () => {
            const results = [];
            for (const { family, weights } of [
              { family: 'Geist', weights: [400, 500, 600, 700] },
              { family: 'Geist Mono', weights: [400, 500] },
            ]) {
              for (const weight of weights) {
                const faces = await document.fonts.load(`${weight} 16px "${family}"`, 'CADE Āž');
                results.push({
                  family, weight,
                  faces: faces.map((face) => ({
                    family: face.family.replace(/["']/g, ''),
                    status: face.status, weight: face.weight, style: face.style,
                  })),
                });
              }
            }
            await document.fonts.ready;
            return results;
          });
          for (const { family, faces } of loaded) {
            expect(faces).toHaveLength(2);
            for (const face of faces) {
              expect(face).toEqual({ family, status: 'loaded', weight: '100 900', style: 'normal' });
            }
          }
          expect(await page.locator('body').evaluate((el) => getComputedStyle(el).fontFamily)).toMatch(/^Geist,/);
          expect(await page.getByText('React · spec · PR', { exact: true }).evaluate((el) => getComputedStyle(el).fontFamily))
            .toMatch(/^"Geist Mono",/);
          await testInfo.attach(`start-fonts-${tauri ? 'tauri' : 'browser'}-${theme}`, {
            body: await page.screenshot(), contentType: 'image/png',
          });
          if (!tauri && testInfo.project.name === 'default-window') {
            await page.setViewportSize({ width: 1920, height: 1080 });
            await testInfo.attach(`start-fonts-browser-${theme}-1920`, {
              body: await page.screenshot(), contentType: 'image/png',
            });
            await page.setViewportSize({ width: 1440, height: 900 });
          }
        }
        expect(fontResponses).toHaveLength(4);
        expect(fontResponses.every(({ url, status }) => new URL(url).origin === new URL(page.url()).origin && status === 200))
          .toBe(true);
        expect(fontResponses.map(({ url }) => new URL(url).pathname.split('/').pop()).sort()).toEqual([
          'geist-latin-ext.woff2', 'geist-latin.woff2', 'geist-mono-latin-ext.woff2', 'geist-mono-latin.woff2',
        ]);
        expect(requests.filter((url) => new URL(url).origin !== new URL(page.url()).origin)).toEqual([]);
        expect(errors).toEqual([]);
      } finally {
        await context.close();
      }
    }
  });

  test('Start_OutputCard_IsNamedByTypeAndDescribedByBlurb', async ({ page }) => {
    await page.goto('/');

    const card = page.getByRole('button', { name: 'Architecture', exact: true });

    await expect(card).toHaveAccessibleDescription('Azure services with real configs, drawn with Azure’s own icons. Bicep · Terraform · PR');
  });

  test('Start_Loaded_UsesTheAppIconAsFavicon', async ({ page }) => {
    await page.goto('/');

    const href = await page.locator('link[rel="icon"]').getAttribute('href');
    const icon = await page.request.get(href ?? '');

    expect(icon.ok()).toBe(true);
    expect(icon.headers()['content-type']).toBe('image/png');
  });

  test('Start_AnyWindowSize_HasNoHorizontalOverflow', async ({ page }) => {
    await page.goto('/');

    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);

    expect(overflow).toBeLessThanOrEqual(0);
  });

  test('Start_PromptFocused_ShowsFocusRingInSelectionColor', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Send feedback' }).focus();
    await page.keyboard.press('Shift+Tab');
    await page.keyboard.press('Tab');

    const outline = await page.getByRole('button', { name: 'Send feedback' }).evaluate((el) => getComputedStyle(el).outlineColor);
    expect(outline).toBe(tokenRgb('accent-selection', 'dark'));
  });
});

test.describe('Theme', () => {
  const background = (page: import('@playwright/test').Page) => page.evaluate(() => getComputedStyle(document.body).backgroundColor);

  test('Theme_FirstLaunch_IsDark', async ({ page }) => {
    await page.goto('/');

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
    expect(await background(page)).toBe(tokenRgb('bg-app', 'dark'));
  });

  test('Theme_LightChosen_RepaintsAndSurvivesReload', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('radio', { name: 'Light' }).click();
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
    expect(await background(page)).toBe(tokenRgb('bg-app', 'light'));

    await page.reload();
    await expect(page.getByRole('radio', { name: 'Light' })).toHaveAttribute('aria-checked', 'true');
    expect(await background(page)).toBe(tokenRgb('bg-app', 'light'));
  });

  for (const scheme of ['light', 'dark'] as const) {
    test(`Theme_SystemChosen_FollowsOs_${scheme}`, async ({ page }) => {
      await page.emulateMedia({ colorScheme: scheme });
      await page.goto('/');

      await page.getByRole('radio', { name: 'System' }).click();

      await expect(page.locator('html')).toHaveAttribute('data-theme', 'system');
      expect(await background(page)).toBe(tokenRgb('bg-app', scheme));
    });
  }
});
