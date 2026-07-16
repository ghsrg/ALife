import { expect, test } from '@playwright/test';

test('Control Center starts a live Runner run and receives frames', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto('/');

  await expect(page.getByText('Connected', { exact: true })).toBeVisible({ timeout: 15000 });
  await expect(page.getByLabel('Scenario')).toHaveValue(/.+/);

  await page.getByRole('button', { name: 'Play live run' }).click();

  await expect(page.getByText(/Live Tick [1-9]/)).toBeVisible({ timeout: 15000 });
  await expect(page.getByLabel('Cell Inspector')).toContainText(/ID|No cell selected/);

  await page.getByRole('button', { name: 'Pause live run' }).click();
  await expect(page.getByRole('button', { name: 'Resume live run' })).toBeEnabled({ timeout: 10000 });

  await page.getByRole('button', { name: 'Step N: one committed tick' }).click();
  await expect(page.getByRole('button', { name: 'Resume live run' })).toBeEnabled({ timeout: 10000 });

  await page.getByRole('button', { name: 'Resume live run' }).click();
  await expect(page.getByRole('button', { name: 'Pause live run' })).toBeEnabled({ timeout: 10000 });

  await page.getByRole('button', { name: 'Stop live run' }).click();
});
