import { expect, test } from '@playwright/test';

const REFERENCE_WIDTH = 1440;
const REFERENCE_HEIGHT = 900;

test.use({
  viewport: { width: REFERENCE_WIDTH, height: REFERENCE_HEIGHT },
  deviceScaleFactor: 1,
});

test('reference UI baseline', async ({ page }) => {
  await page.goto('/target-route');

  await page.evaluate(() => document.fonts.ready);
  await page.waitForFunction(() =>
    Array.from(document.images).every((image) => image.complete),
  );

  await page.addStyleTag({
    content: `
      *, *::before, *::after {
        animation-duration: 0s !important;
        animation-delay: 0s !important;
        transition: none !important;
        caret-color: transparent !important;
      }
    `,
  });

  await expect(page).toHaveScreenshot('reference-ui-baseline.png', {
    animations: 'disabled',
    caret: 'hide',
    fullPage: true,
  });
});
