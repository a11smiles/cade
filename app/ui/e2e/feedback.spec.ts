import { expect, test, type Page } from '@playwright/test';
import { FORMS, type Feedback, type IssueDraft } from '../src/feedback/types';
import { FORMS as FORM_METADATA } from '../src/feedback/forms.generated';
import { ipcCalls, mockTauri, type IpcCall } from './tauri';
import { tokenRgb } from './tokens';

type ControlledOutcome = { preview: true } | { ok: string } | { err: { Message: string } };

async function finishRender(page: Page) {
  await page.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
}

async function respondToFeedback(page: Page, cmd: string, index: number, outcome: ControlledOutcome) {
  const result = await page.evaluate(({ cmd, index, outcome }) => {
    return (window as unknown as {
      __cadeRespond: (cmd: string, index: number, outcome: ControlledOutcome) => Promise<IpcCall['preview']>;
    }).__cadeRespond(cmd, index, outcome);
  }, { cmd, index, outcome });
  // Flush promise handlers and React's render without wall-clock race delays.
  await finishRender(page);
  return result;
}

async function releasePreview(page: Page, index: number): Promise<IssueDraft> {
  const result = await respondToFeedback(page, 'preview_feedback', index, { preview: true });
  if (!result || !('ok' in result)) throw new Error('The Rust preview rejected the completed synthetic feedback.');
  return result.ok;
}

async function latestRustDraft(page: Page): Promise<IssueDraft> {
  await expect.poll(async () => (await ipcCalls(page, 'preview_feedback')).at(-1)?.preview).toHaveProperty('ok');
  const result = (await ipcCalls(page, 'preview_feedback')).at(-1)?.preview;
  if (!result || !('ok' in result)) throw new Error('No successful Rust draft for the current feedback.');
  return result.ok;
}

async function feedbackRequestIndex(page: Page, cmd: string, expected: Record<string, unknown>) {
  await expect.poll(async () => (await ipcCalls(page, cmd)).at(-1)?.args.feedback).toMatchObject(expected);
  return (await ipcCalls(page, cmd)).length - 1;
}

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

async function openReviewedBug(page: Page) {
  const dialog = await openFeedback(page);
  await fillBug(page);
  const index = await feedbackRequestIndex(page, 'preview_feedback', {
    title: 'Canvas freezes',
    answers: { steps: 'Open a big file.' },
  });
  const previewCall = (await ipcCalls(page, 'preview_feedback'))[index];
  if (!previewCall) throw new Error('No completed bug input reached preview_feedback');
  const draft = await releasePreview(page, index);
  await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
  await expect(dialog.getByLabel('Labels', { exact: true })).toHaveText(draft.labels.join(', '));
  return { dialog, previewCall, draft };
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

  test('Feedback_KeyboardOnly_FocusStaysInsideAndEscapeReturnsToOpener', async ({ page }) => {
    await page.goto('/');
    const opener = page.getByRole('button', { name: 'Send feedback' });

    await opener.focus();
    await page.keyboard.press('Enter');
    const dialog = page.getByRole('dialog', { name: 'Send feedback' });
    await expect(dialog.getByRole('button', { name: 'Close' })).toBeFocused();

    for (let i = 0; i < 20; i++) {
      await page.keyboard.press('Tab');
      expect(await dialog.evaluate((el) => el.contains(document.activeElement))).toBe(true);
    }
    for (let i = 0; i < 5; i++) {
      await page.keyboard.press('Shift+Tab');
      expect(await dialog.evaluate((el) => el.contains(document.activeElement))).toBe(true);
    }

    await page.keyboard.press('Escape');
    await expect(page.getByRole('dialog')).toHaveCount(0);
    await expect(opener).toBeFocused();
  });

  test('Feedback_LogsOption_IsDisabledAndSaysItIsUnavailable', async ({ page }) => {
    const dialog = await openFeedback(page);

    const logs = dialog.getByRole('checkbox', { name: /Recent logs, scrubbed/ });

    await expect(logs).toBeDisabled();
    await expect(logs).not.toBeChecked();
    await expect(logs).toHaveAccessibleDescription('Recent logs are not available. No logs will be sent.');
  });

  test('Feedback_Opened_FitsTheWindow', async ({ page }) => {
    const dialog = await openFeedback(page);

    const box = await dialog.boundingBox();
    const viewport = page.viewportSize();
    expect(box && viewport).toBeTruthy();
    if (!box || !viewport) throw new Error('The feedback dialog or viewport has no bounds.');
    expect(box.x).toBeGreaterThanOrEqual(0);
    expect(box.y).toBeGreaterThanOrEqual(0);
    expect(box.x + box.width).toBeLessThanOrEqual(viewport.width);
    expect(box.y + box.height).toBeLessThanOrEqual(viewport.height);
    await expect(dialog.getByRole('button', { name: 'Submit bug' })).toBeInViewport();
  });
});

test.describe('Feedback in the desktop app (mock IPC)', () => {
  test('Feedback_TypedRightAfterOpening_KeepsEveryAnswer', async ({ page }) => {
    await mockTauri(page);
    for (let i = 0; i < 15; i++) {
      const dialog = await openFeedback(page);
      await fillBug(page);

      await expect(dialog.getByRole('textbox', { name: /What happened\?/ })).toHaveValue('It froze.', { timeout: 1000 });
      await expect(dialog.getByRole('textbox', { name: /Steps to reproduce/ })).toHaveValue('Open a big file.', { timeout: 1000 });
      await expect(dialog.getByRole('button', { name: 'Submit bug' })).toBeEnabled({ timeout: 1000 });
    }
  });

  // The parity reds precede implementation. Transport is controlled, but every
  // successful draft now comes from the real Rust preview executable.
  test.describe('Feedback next-phase contracts', () => {
    for (const theme of ['dark', 'light'] as const) {
      test.describe(theme, () => {
        test.beforeEach(async ({ page }) => {
          await page.addInitScript((theme) => localStorage.setItem('cade.theme', theme), theme);
          await mockTauri(page, undefined, ['preview_feedback', 'submit_feedback']);
        });

        for (const kind of ['bug', 'feature-request', 'comment'] as const) {
          test(`Feedback_${kind}MetadataControls_PreviewsCanonicalAnswerKeysAndOptions`, async ({ page }) => {
            const dialog = await openFeedback(page);
            await dialog.getByRole('radio', { name: new RegExp(`^${FORMS[kind].name}`) }).click();
            await dialog.getByRole('textbox', { name: 'Title', exact: true }).fill('Synthetic parity report');
            const answers: Record<string, string> = {};

            for (const field of FORM_METADATA[kind].body) {
              if (!('id' in field) || ['app_version', 'os', 'gpu', 'policy_version', 'screenshot_url'].includes(field.id)) continue;
              const name = new RegExp(field.attributes.label.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'));
              if ('options' in field.attributes) {
                const multiple = 'multiple' in field.attributes && field.attributes.multiple;
                const control = dialog.getByRole(multiple ? 'listbox' : 'combobox', { name });
                await control.scrollIntoViewIfNeeded();
                await expect(control).toBeInViewport();
                await expect(control.getByRole('option')).toHaveText(
                  multiple ? [...field.attributes.options] : ['Not specified', ...field.attributes.options],
                );
                await expect(control).toHaveAttribute('name', field.id);
                if (multiple) {
                  await control.selectOption(['Image', 'App']);
                  answers[field.id] = 'App, Image';
                } else {
                  const value = field.attributes.options[0];
                  await control.selectOption(value);
                  answers[field.id] = value;
                }
              } else {
                const control = dialog.getByRole('textbox', { name });
                await control.scrollIntoViewIfNeeded();
                await expect(control).toBeInViewport();
                await expect(control).toHaveAttribute('name', field.id);
                const value = `Synthetic ${field.id}`;
                await control.fill(value);
                answers[field.id] = value;
              }
            }

            const index = await feedbackRequestIndex(page, 'preview_feedback', { kind, title: 'Synthetic parity report', answers });
            expect((await ipcCalls(page, 'preview_feedback')).at(-1)?.args.feedback).toMatchObject({ answers });
            expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
            const reviewed = (await ipcCalls(page, 'preview_feedback'))[index];
            const draft = await releasePreview(page, index);
            expect(draft.labels).toEqual([...FORM_METADATA[kind].labels]);
            await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
            await expect(dialog.getByLabel('Labels', { exact: true })).toHaveText(draft.labels.join(', '));
            await dialog.getByLabel('Issue preview').scrollIntoViewIfNeeded();
            await expect(dialog.getByLabel('Issue preview')).toBeInViewport();
            await dialog.getByRole('button', { name: FORMS[kind].cta, exact: true }).click();
            expect((await ipcCalls(page, 'submit_feedback')).map(({ args }) => args)).toEqual([reviewed?.args]);
            for (const role of ['textbox', 'radio', 'checkbox', 'listbox', 'combobox'] as const) {
              for (const control of await dialog.getByRole(role).all()) await expect(control).toBeDisabled();
            }
          });
        }

        test('Feedback_RustDraftLabels_ShowsExactReturnedLabelsBesideUnchangedDraft', async ({ page }) => {
          const { dialog, draft } = await openReviewedBug(page);

          await expect(dialog.getByLabel('Labels', { exact: true })).toHaveText(draft.labels.join(', '));
          await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
        });

        test('Feedback_SubmitPending_DisablesEditingAndRetainsReviewedSnapshot', async ({ page }) => {
          const { dialog, previewCall } = await openReviewedBug(page);
          await dialog.getByRole('button', { name: 'Submit bug', exact: true }).click();
          await feedbackRequestIndex(page, 'submit_feedback', { ...previewCall.args.feedback as Feedback });

          for (const role of ['textbox', 'radio', 'checkbox', 'listbox'] as const) {
            for (const control of await dialog.getByRole(role).all()) await expect.soft(control).toBeDisabled({ timeout: 100 });
          }
          await expect(dialog.getByRole('textbox', { name: 'Title', exact: true })).toHaveValue('Canvas freezes');
          expect((await ipcCalls(page, 'submit_feedback')).map(({ args }) => args)).toEqual([previewCall.args]);
          await expect(dialog.getByRole('button', { name: 'Sending…', exact: true })).toBeDisabled();
        });

        test('Feedback_SubmitRejected_ExplainsRetainedEntriesWithoutClaimingNotSent', async ({ page }) => {
          const { dialog } = await openReviewedBug(page);
          await dialog.getByRole('button', { name: 'Submit bug', exact: true }).click();
          const index = await feedbackRequestIndex(page, 'submit_feedback', { title: 'Canvas freezes' });
          await respondToFeedback(page, 'submit_feedback', index, { err: { Message: 'Synthetic response interrupted' } });

          await expect(dialog.getByText('Couldn’t submit your issue. Your entries are still in this form.')).toBeVisible();
          await expect(dialog.getByText('Couldn’t submit your issue. Your entries are still in this form.')).toBeInViewport();
          await expect(dialog.getByText(/not sent/i)).toHaveCount(0);
          await expect(dialog.getByRole('textbox', { name: 'Title', exact: true })).toHaveValue('Canvas freezes');
          await expect(dialog.getByRole('textbox', { name: /What happened\?/ })).toHaveValue('It froze.');
          await expect(dialog.getByRole('textbox', { name: /Steps to reproduce/ })).toHaveValue('Open a big file.');
          await expect(dialog.getByRole('textbox', { name: 'Title', exact: true })).toBeEnabled();
        });

        test('Feedback_UnavailableDetails_ExplainsEachOmissionWithoutInventedValues', async ({ page }) => {
          const dialog = await openFeedback(page);
          await fillBug(page);

          for (const label of ['GPU and renderer', 'Policy version']) {
            const control = dialog.getByRole('checkbox', { name: label, exact: true });
            await expect.soft(control).toBeDisabled({ timeout: 100 });
            await expect.soft(control).not.toBeChecked({ timeout: 100 });
            await expect.soft(control).toHaveAccessibleDescription('Not available. This detail will not be sent.', { timeout: 100 });
          }
          const logs = dialog.getByRole('checkbox', { name: 'Recent logs, scrubbed', exact: true });
          await expect(logs).toBeDisabled();
          await expect(logs).not.toBeChecked();
          await expect.soft(logs).toHaveAccessibleDescription('Recent logs are not available. No logs will be sent.', { timeout: 100 });
          for (const note of await dialog.getByText('Not available. This detail will not be sent.', { exact: true }).all()) {
            await note.scrollIntoViewIfNeeded();
            await expect(note).toBeInViewport();
          }
          const logsNote = dialog.getByText('Recent logs are not available. No logs will be sent.', { exact: true });
          await logsNote.scrollIntoViewIfNeeded();
          await expect(logsNote).toBeInViewport();
          expect((await ipcCalls(page, 'preview_feedback')).at(-1)?.args.feedback).toMatchObject({
            environment: { gpu: null, policy_version: null, logs: null },
          });
        });

        test('FeedbackDialog_PopulatedCommentReplyTab_ShowsCompleteScreenshotUrlOutline', async ({ page }) => {
          await page.setViewportSize({ width: 1100, height: 700 });
          await page.emulateMedia({ colorScheme: 'light', forcedColors: 'none', reducedMotion: 'reduce' });
          const dialog = await openFeedback(page);
          await dialog.getByRole('radio', { name: new RegExp(`^${FORMS.comment.name}`) }).click();
          const title = dialog.getByRole('textbox', { name: 'Title', exact: true });
          await title.fill('Synthetic measurement report');
          for (const question of FORMS.comment.questions) {
            const name = new RegExp(question.heading.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'));
            if (question.type === 'dropdown') {
              await dialog.getByRole(question.multiple ? 'listbox' : 'combobox', { name })
                .selectOption(question.multiple ? ['App', 'System', 'Other'] : question.options?.at(-1) ?? '');
            } else {
              await dialog.getByRole('textbox', { name }).fill(`Synthetic ${question.id}; measurement only`);
            }
          }
          const screenshot = dialog.getByRole('textbox', { name: /Screenshot URL/ });
          await screenshot.fill('https://example.invalid/measurement.png');
          const index = await feedbackRequestIndex(page, 'preview_feedback', {
            kind: 'comment', screenshot_url: 'https://example.invalid/measurement.png',
          });
          const draft = await releasePreview(page, index);
          await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);

          // Preserve the designer's capture-scroll setup before actual keyboard entry.
          await screenshot.focus();
          await title.scrollIntoViewIfNeeded();
          await screenshot.scrollIntoViewIfNeeded();
          await dialog.getByRole('combobox', { name: /Would you like a reply\?/ }).focus();
          await page.keyboard.press('Tab');
          // Read immediately: no focus override, corrective scroll, or polling.
          const geometry = await screenshot.evaluate((element) => {
            const rect = element.getBoundingClientRect();
            const style = getComputedStyle(element);
            const clip = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
            for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
              const bounds = ancestor.getBoundingClientRect();
              const ancestorStyle = getComputedStyle(ancestor);
              if (/auto|scroll|hidden|clip/.test(ancestorStyle.overflowX)) {
                clip.left = Math.max(clip.left, bounds.left + ancestor.clientLeft);
                clip.right = Math.min(clip.right, bounds.left + ancestor.clientLeft + ancestor.clientWidth);
              }
              if (/auto|scroll|hidden|clip/.test(ancestorStyle.overflowY)) {
                clip.top = Math.max(clip.top, bounds.top + ancestor.clientTop);
                clip.bottom = Math.min(clip.bottom, bounds.top + ancestor.clientTop + ancestor.clientHeight);
              }
            }
            const extent = Math.max(0, parseFloat(style.outlineWidth) + parseFloat(style.outlineOffset));
            const centerHit = document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2);
            const textRange = document.createRange();
            textRange.selectNodeContents(element);
            const textRects = Array.from(textRange.getClientRects()).filter((r) => r.width > 0 && r.height > 0);
            return {
              activeId: document.activeElement?.id,
              focused: document.activeElement === element,
              focusVisible: element.matches(':focus-visible'),
              rect: rect.toJSON(),
              clip,
              rectVisible: rect.width > 0 && rect.height > 0 && rect.left >= clip.left
                && rect.right <= clip.right && rect.top >= clip.top && rect.bottom <= clip.bottom,
              textVisible: textRects.every((r) => r.left >= clip.left && r.right <= clip.right
                && r.top >= clip.top && r.bottom <= clip.bottom),
              centerHit: centerHit === element || (centerHit !== null && element.contains(centerHit)),
              outline: {
                color: style.outlineColor, width: style.outlineWidth,
                offset: style.outlineOffset, style: style.outlineStyle,
              },
              clippedPixels: {
                left: Math.max(0, clip.left - (rect.left - extent)),
                top: Math.max(0, clip.top - (rect.top - extent)),
                right: Math.max(0, rect.right + extent - clip.right),
                bottom: Math.max(0, rect.bottom + extent - clip.bottom),
              },
            };
          });
          expect(geometry).toMatchObject({
            activeId: 'fb-screenshot', focused: true, focusVisible: true,
            rectVisible: true, textVisible: true, centerHit: true,
            outline: { color: tokenRgb('accent-selection', theme), width: '2px', offset: '2px', style: 'solid' },
          });
          expect(geometry.clippedPixels, JSON.stringify(geometry)).toEqual({ left: 0, top: 0, right: 0, bottom: 0 });
        });

        test('Feedback_ScreenshotUrlEdited_PreviewsTopLevelUrlWithoutSending', async ({ page }) => {
          const dialog = await openFeedback(page);
          await fillBug(page);
          const screenshot = dialog.getByRole('textbox', { name: /Screenshot URL/ });
          const url = 'https://example.com/synthetic-feedback.png';
          await expect(screenshot).toBeVisible();
          await screenshot.fill(url);

          const index = await feedbackRequestIndex(page, 'preview_feedback', { screenshot_url: url });
          const feedback = (await ipcCalls(page, 'preview_feedback')).at(-1)?.args.feedback;
          expect(feedback).toMatchObject({ screenshot_url: url });
          expect(feedback).not.toHaveProperty('answers.screenshot_url');
          expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
          const draft = await releasePreview(page, index);
          expect(draft.body).toContain(`![screenshot](${url})`);
          await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
        });
      });
    }
  });

  test('Feedback_FormEdited_PreviewShowsWhatGitHubWillReceive', async ({ page }) => {
    await mockTauri(page);
    const dialog = await openFeedback(page);

    await fillBug(page);

    const draft = await latestRustDraft(page);
    expect(draft.title).toBe('[Bug] Canvas freezes');
    expect(draft.body).toContain('### What happened?\n\nIt froze.\n\n### Steps to reproduce\n\nOpen a big file.\n\n### What did you expect?\n\n_No response_');
    await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
    await expect(dialog.getByLabel('Labels', { exact: true })).toHaveText(draft.labels.join(', '));
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

  // Two themes × the existing minimum/default projects, not the full kind/size matrix.
  for (const theme of ['dark', 'light'] as const) {
    test.describe(`Feedback preview authorization and success focus (${theme})`, () => {
      test.beforeEach(async ({ page }) => {
        await page.addInitScript((theme) => localStorage.setItem('cade.theme', theme), theme);
        await mockTauri(page, undefined, ['preview_feedback', 'submit_feedback']);
      });

      test('Feedback_CurrentEditPreviewPending_DisablesSubmitWithoutSending', async ({ page }) => {
        const { dialog } = await openReviewedBug(page);

        await dialog.getByRole('textbox', { name: 'Title', exact: true }).fill('New unpreviewed content');
        await feedbackRequestIndex(page, 'preview_feedback', { title: 'New unpreviewed content' });

        const submit = dialog.getByRole('button', { name: 'Submit bug', exact: true });
        await expect.soft(submit).toBeDisabled();
        await submit.focus();
        await page.keyboard.press('Enter');
        expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
      });

      test('Feedback_CurrentPreviewRejected_DisablesSubmitAndKeepsEntries', async ({ page }) => {
        const { dialog } = await openReviewedBug(page);
        await dialog.getByRole('textbox', { name: 'Title', exact: true }).fill('New unpreviewed content');
        const index = await feedbackRequestIndex(page, 'preview_feedback', { title: 'New unpreviewed content' });

        await respondToFeedback(page, 'preview_feedback', index, { err: { Message: 'Feedback preview is temporarily unavailable' } });

        await expect(dialog.getByRole('textbox', { name: 'Title', exact: true })).toHaveValue('New unpreviewed content');
        await expect(dialog.getByRole('textbox', { name: /What happened\?/ })).toHaveValue('It froze.');
        await expect(dialog.getByRole('textbox', { name: /Steps to reproduce/ })).toHaveValue('Open a big file.');
        const submit = dialog.getByRole('button', { name: 'Submit bug', exact: true });
        await expect.soft(submit).toBeDisabled();
        await submit.focus();
        await page.keyboard.press('Enter');
        expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
      });

      test('Feedback_CurrentPreviewRejected_OffersExplicitRetryForTheSameEntries', async ({ page }) => {
        const { dialog } = await openReviewedBug(page);
        await dialog.getByRole('textbox', { name: 'Title', exact: true }).fill('Retry this report');
        const index = await feedbackRequestIndex(page, 'preview_feedback', { title: 'Retry this report' });
        const failedCall = (await ipcCalls(page, 'preview_feedback'))[index];
        await respondToFeedback(page, 'preview_feedback', index, { err: { Message: 'Feedback preview is temporarily unavailable' } });

        await expect.soft(dialog.getByText('Couldn’t preview your issue. Your entries are still in this form.')).toBeVisible();
        const retry = dialog.getByRole('button', { name: 'Retry preview', exact: true });
        await expect(retry).toBeVisible();
        const before = (await ipcCalls(page, 'preview_feedback')).length;
        await retry.click();

        await expect.poll(async () => (await ipcCalls(page, 'preview_feedback')).length).toBe(before + 1);
        expect((await ipcCalls(page, 'preview_feedback')).at(-1)?.args).toEqual(failedCall?.args);
        await expect(dialog.getByRole('button', { name: 'Submit bug', exact: true })).toBeDisabled();
        const draft = await releasePreview(page, before);
        expect(draft.title).toBe('[Bug] Retry this report');
        await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
        await expect(dialog.getByRole('button', { name: 'Submit bug', exact: true })).toBeEnabled();
        expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
      });

      test('Feedback_EditDuringSubmit_KeepsTheDisplayedPreviewInputSnapshot', async ({ page }) => {
        const { dialog, previewCall } = await openReviewedBug(page);

        await dialog.getByRole('button', { name: 'Submit bug', exact: true }).click();
        await feedbackRequestIndex(page, 'submit_feedback', { ...previewCall.args.feedback as Feedback });
        const title = dialog.getByRole('textbox', { name: 'Title', exact: true });
        await expect(title).toBeDisabled();
        // Playwright fill intentionally waits for editability; exercise a real
        // attempted keyboard edit instead, without bypassing the disabled state.
        await title.evaluate((element) => (element as HTMLInputElement).focus());
        await page.keyboard.type('Edited while sending');

        await expect(title).toHaveValue('Canvas freezes');
        const submissions = await ipcCalls(page, 'submit_feedback');
        expect(submissions).toHaveLength(1);
        expect(submissions[0]?.args).toEqual(previewCall.args);
        await expect(dialog.getByRole('button', { name: 'Sending…', exact: true })).toBeDisabled();
      });

      test('Feedback_SubmitSucceeds_FocusesSuccessHeadingAndPreservesModalKeyboardNavigation', async ({ page }) => {
        const { dialog } = await openReviewedBug(page);
        const opener = page.getByRole('button', { name: 'Send feedback', exact: true });
        await dialog.getByRole('button', { name: 'Submit bug', exact: true }).focus();
        await page.keyboard.press('Enter');
        const index = await feedbackRequestIndex(page, 'submit_feedback', { title: 'Canvas freezes' });

        await respondToFeedback(page, 'submit_feedback', index, { ok: 'https://github.com/a11smiles/cade/issues/600' });

        // Check the success transition before any keypress or focus repair.
        expect.soft(await dialog.evaluate((element) => element.contains(document.activeElement))).toBe(true);
        await expect.soft(dialog.getByRole('status').getByRole('heading', { name: 'Issue opened', exact: true })).toBeFocused();
        const closeSuccess = dialog.getByRole('status').getByRole('button', { name: 'Close', exact: true });
        await closeSuccess.focus();
        await page.keyboard.press('Tab');
        await expect(dialog.getByRole('button', { name: 'Close', exact: true }).first()).toBeFocused();
        await page.keyboard.press('Shift+Tab');
        await expect(closeSuccess).toBeFocused();
        await page.keyboard.press('Escape');
        await expect(dialog).toHaveCount(0);
        await expect(opener).toBeFocused();
      });

      test('Feedback_EqualContentRerenders_RequestsOnlyDistinctEdits', async ({ page }) => {
        const dialog = await openFeedback(page);
        await dialog.getByRole('radio', { name: /^Feature request/ }).click();
        await dialog.getByRole('textbox', { name: 'Title', exact: true }).fill('Export to Figma');
        // The facade obtains requiredness and IDs from the generated YAML metadata.
        const required = FORMS['feature-request'].questions.filter((question) => question.required);
        for (const question of required) {
          await dialog.getByRole('textbox', { name: `${question.heading} required`, exact: true }).fill(`Synthetic ${question.heading}`);
        }
        const first = required[0];
        if (!first) throw new Error('The feature form has no required question');
        const index = await feedbackRequestIndex(page, 'preview_feedback', {
          kind: 'feature-request',
          title: 'Export to Figma',
          answers: Object.fromEntries(required.map((question) => [question.id, `Synthetic ${question.heading}`])),
        });
        const draft = await releasePreview(page, index);
        expect(draft.title).toBe('[Feature] Export to Figma');
        await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
        const before = (await ipcCalls(page, 'preview_feedback')).length;
        const answer = dialog.getByRole('textbox', { name: `${first.heading} required`, exact: true });

        await answer.pressSequentially('ab');
        const edited = await feedbackRequestIndex(page, 'preview_feedback', {
          answers: { [first.id]: `Synthetic ${first.heading}ab` },
        });
        const calls = await ipcCalls(page, 'preview_feedback');
        expect(calls).toHaveLength(before + 2);
        expect(calls[before]?.args.feedback).toMatchObject({
          answers: { [first.id]: `Synthetic ${first.heading}a` },
        });
        expect(new Set(calls.slice(before).map((call) => JSON.stringify(call.args.feedback))).size).toBe(2);
        const updated = await releasePreview(page, edited);
        expect(updated.body).toContain(`Synthetic ${first.heading}ab`);
        await expect(dialog.getByLabel('Issue preview')).toHaveText(`${updated.title}\n\n${updated.body}`);
        await dialog.getByRole('radio', { name: /^Feature request/ }).click();
        await answer.fill(`Synthetic ${first.heading}ab`);
        await finishRender(page);

        expect(await ipcCalls(page, 'preview_feedback')).toHaveLength(before + 2);
        expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
      });

      test('Feedback_PreviewResponsesReordered_KeepsTheLatestContent', async ({ page }) => {
        const { dialog } = await openReviewedBug(page);
        const title = dialog.getByRole('textbox', { name: 'Title', exact: true });
        await title.fill('Older');
        const older = await feedbackRequestIndex(page, 'preview_feedback', { title: 'Older' });
        await title.fill('Newer');
        const newer = await feedbackRequestIndex(page, 'preview_feedback', { title: 'Newer' });
        const draft = await releasePreview(page, newer);
        expect(draft.title).toBe('[Bug] Newer');
        await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
        const stale = await releasePreview(page, older);
        expect(stale.title).toBe('[Bug] Older');

        await expect(dialog.getByLabel('Issue preview')).toHaveText(`${draft.title}\n\n${draft.body}`);
        await expect(dialog.getByRole('button', { name: 'Submit bug', exact: true })).toBeEnabled();
        expect(await ipcCalls(page, 'submit_feedback')).toHaveLength(0);
      });
    });
  }

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

    await expect(dialog.getByRole('alert')).toHaveText('Couldn’t submit your issue. Your entries are still in this form. no GitHub token stored');
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
        answers: {
          problem: 'Handoff.',
          proposal: 'Export frames.',
          output: '',
          alternatives: '',
          beneficiaries: '',
          contribute: '',
        },
      },
    });
  });
});
