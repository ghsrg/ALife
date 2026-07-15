import { expect, test } from '@playwright/test';

test('UI-1A Monitor opens at 1024x768 with fixture viewer and Inspector', async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  await page.goto('/');

  await expect(page.getByRole('heading', { name: 'ALife Control Center' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Monitor' })).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByText('UI-1A Deterministic Fixture')).toBeVisible();
  await expect(page.getByText('Tick 128')).toBeVisible();
  await expect(page.getByLabel('World Viewer')).toHaveAttribute('data-ready', 'true');
  await expect(page.getByLabel('Cell Inspector')).toContainText('cell-a');

  await page.getByLabel('Select cell-c').click();
  await expect(page.getByLabel('Cell Inspector')).toContainText('cell-c');
  await expect(page.getByLabel('Cell Inspector')).toContainText('resource-rich region');

  await page.getByRole('button', { name: 'Switch to light theme' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');

  await page.getByRole('button', { name: 'Export viewer PNG' }).click();
  await expect(page.getByRole('status')).toContainText('PNG ready');
});
