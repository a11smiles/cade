import { expect, test } from '@playwright/test';
import { tokenRgb } from './tokens';

test.describe('Start screen', () => {
  test('Start_Loaded_ShowsEveryOutputTypeWithoutErrors', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (e) => errors.push(e.message));
    page.on('console', (m) => m.type() === 'error' && !m.text().includes('fonts.g') && errors.push(m.text()));

    await page.goto('/');

    await expect(page).toHaveTitle('CADE');
    await expect(page.getByRole('heading', { level: 1, name: 'What are you making?' })).toBeVisible();
    for (const label of ['App', 'Image', 'Deck', 'Document', 'Motion', 'Diagram', 'Architecture', 'Data model']) {
      await expect(page.getByRole('button', { name: label, exact: true })).toBeVisible();
    }
    await expect(page.getByText('Local only')).toBeVisible();
    expect(errors).toEqual([]);
  });

  test('Start_OutputCard_IsNamedByTypeAndDescribedByBlurb', async ({ page }) => {
    await page.goto('/');

    const card = page.getByRole('button', { name: 'Architecture', exact: true });

    await expect(card).toHaveAccessibleDescription('Azure services with real configs, drawn with Azure’s own icons. Bicep · Terraform · PR');
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
