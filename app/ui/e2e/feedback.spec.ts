import { expect, test, type Page } from '@playwright/test';
import { ipcCalls, mockTauri } from './tauri';

async function openFeedback(page: Page) {
  await page.goto('/');
  await page.getByRole('button', { name: 'Send feedback' }).click();
  return page.getByRole('dialog', { name: 'Send feedback' });
}

async function fillBug(page: Page) {
  await page.getByRole('textbox', { name: 'Title' }).fill('Canvas freezes');
  await page.getByRole('textbox', { name: /What happened\?/ }).fill('It froze.');
  await page.getByRole('textbox', { name: /Steps to reproduce/ }).fill('Open a big file.');
}

test.describe('Feedback outside the desktop app', () => {
  test('Feedback_PlainBrowser_ExplainsWhereToSendAndCannotSubmit', async ({ page }) => {
    const dialog = await openFeedback(page);

    await fillBug(page);

    await expect(dialog.getByText('Open the desktop app to send.')).toBeVisible();
    await expect(dialog.getByRole('button', { name: 'Submit bug' })).toBeDisabled();
    await expect(dialog.getByLabel('Issue preview')).toHaveCount(0);
  });

  test('Feedback_KeyboardOnly_OpensAndCloses', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Send feedback' }).focus();
    await page.keyboard.press('Enter');
    await expect(page.getByRole('dialog', { name: 'Send feedback' })).toBeVisible();

    await page.getByRole('button', { name: 'Cancel' }).focus();
    await page.keyboard.press('Enter');
    await expect(page.getByRole('dialog')).toHaveCount(0);
  });

  test('Feedback_Opened_FitsTheWindow', async ({ page }) => {
    const dialog = await openFeedback(page);

    const box = await dialog.boundingBox();
    const viewport = page.viewportSize();
    expect(box && viewport).toBeTruthy();
    expect(box!.x).toBeGreaterThanOrEqual(0);
    expect(box!.y).toBeGreaterThanOrEqual(0);
    expect(box!.x + box!.width).toBeLessThanOrEqual(viewport!.width);
    expect(box!.y + box!.height).toBeLessThanOrEqual(viewport!.height);
    await expect(dialog.getByRole('button', { name: 'Submit bug' })).toBeInViewport();
  });
});

test.describe('Feedback in the desktop app (mock IPC)', () => {
  test('Feedback_FormEdited_PreviewShowsWhatGitHubWillReceive', async ({ page }) => {
    await mockTauri(page);
    const dialog = await openFeedback(page);

    await fillBug(page);

    await expect(dialog.getByLabel('Issue preview')).toHaveText(
      '[Bug] Canvas freezes\n\n### What happened?\n\nIt froze.\n\n### Steps to reproduce\n\nOpen a big file.\n\n### What did you expect?\n\n_No response_',
    );
    await expect(dialog.getByText('Open the desktop app to send.')).toHaveCount(0);
  });

  test('Feedback_OsOptedOut_IsNotSentToRust', async ({ page }) => {
    await mockTauri(page);
    const dialog = await openFeedback(page);
    await fillBug(page);

    await dialog.getByRole('checkbox', { name: 'Operating system' }).uncheck();
    await dialog.getByRole('button', { name: 'Submit bug' }).click();
    await expect(dialog.getByRole('status')).toBeVisible();

    const [submit] = await ipcCalls(page, 'submit_feedback');
    expect(submit?.args).toMatchObject({
      feedback: { kind: 'bug', title: 'Canvas freezes', environment: { os: null, app_version: '0.1.0', logs: null } },
    });
  });

  test('Feedback_SubmitSucceeds_LinksToTheNewIssue', async ({ page }) => {
    await mockTauri(page, { ok: 'https://github.com/a11smiles/cade/issues/600' });
    const dialog = await openFeedback(page);
    await fillBug(page);

    await dialog.getByRole('button', { name: 'Submit bug' }).click();

    const status = dialog.getByRole('status');
    await expect(status).toContainText('Issue opened');
    await expect(status.getByRole('link')).toHaveAttribute('href', 'https://github.com/a11smiles/cade/issues/600');
    await status.getByRole('button', { name: 'Close' }).click();
    await expect(page.getByRole('dialog')).toHaveCount(0);
  });

  test('Feedback_SubmitFails_ShowsReasonAndKeepsTheReport', async ({ page }) => {
    await mockTauri(page, { err: 'no GitHub token stored' });
    const dialog = await openFeedback(page);
    await fillBug(page);

    await dialog.getByRole('button', { name: 'Submit bug' }).click();

    await expect(dialog.getByRole('alert')).toHaveText('no GitHub token stored');
    await expect(dialog.getByRole('textbox', { name: 'Title' })).toHaveValue('Canvas freezes');
    await expect(dialog.getByRole('button', { name: 'Submit bug' })).toBeEnabled();
  });

  test('Feedback_FeatureRequest_SendsKebabCaseKindAndItsQuestions', async ({ page }) => {
    await mockTauri(page);
    const dialog = await openFeedback(page);

    await dialog.getByRole('radio', { name: /^Feature request/ }).click();
    await dialog.getByRole('textbox', { name: 'Title' }).fill('Export to Figma');
    await dialog.getByRole('textbox', { name: /What problem would this solve\?/ }).fill('Handoff.');
    await dialog.getByRole('textbox', { name: /What would you like CADE to do\?/ }).fill('Export frames.');
    await dialog.getByRole('button', { name: 'Submit request' }).click();
    await expect(dialog.getByRole('status')).toBeVisible();

    const [submit] = await ipcCalls(page, 'submit_feedback');
    expect(submit?.args).toMatchObject({
      feedback: {
        kind: 'feature-request',
        answers: [
          { heading: 'What problem would this solve?', text: 'Handoff.' },
          { heading: 'What would you like CADE to do?', text: 'Export frames.' },
          { heading: 'Which output type?', text: '' },
        ],
      },
    });
  });
});
