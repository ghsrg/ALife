import { expect, test, type Page } from '@playwright/test';
import { mkdirSync } from 'node:fs';
import { join } from 'node:path';

const screenshotDir = join(process.cwd(), 'test-results', 'ui-1c-a');

test.describe('UI-1C-A visual acceptance', () => {
  test.beforeAll(() => {
    mkdirSync(screenshotDir, { recursive: true });
  });

  test('1920x1080 dark keeps World View dominant and captures acceptance screenshot', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
    await assertWorldFirstLayout(page);
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Resources');
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Fixture grid');
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Cell size');
    await page.screenshot({ path: join(screenshotDir, '1920x1080-dark.png'), fullPage: true });
  });

  test('1366x768 dark keeps controls usable without incoherent overlap', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 768 });

    await assertWorldFirstLayout(page);
    await expect(page.getByRole('button', { name: 'Play live run' })).toBeVisible();
    await expect(page.getByLabel('World stats')).toBeVisible();
    await page.screenshot({ path: join(screenshotDir, '1366x768-dark.png'), fullPage: true });
  });

  test('1920x1080 light remains usable', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await page.getByRole('button', { name: 'Switch to light theme' }).click();

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
    await assertWorldFirstLayout(page);
    await expect(page.getByLabel('Cell Inspector')).toBeVisible();
    await page.screenshot({ path: join(screenshotDir, '1920x1080-light.png'), fullPage: true });
  });

  test('1920x1080 dark shows selected semantic detail and focus meters', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await expect(page.getByLabel('Selected cell detail label')).toContainText('cell-a');
    await expect(page.getByLabel('Selected cell energy')).toHaveAttribute('aria-valuenow', '82');
    await expect(page.getByLabel('Selected cell integrity')).toHaveAttribute('aria-valuenow', '91');

    await page.screenshot({ path: join(screenshotDir, '1920x1080-semantic-detail.png'), fullPage: true });
  });

  test('1366x768 viewer navigation zooms, resets and keeps cell targets usable', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 768 });

    await page.getByRole('button', { name: 'Zoom in World Viewer' }).click();
    await expect(page.getByLabel('World Viewer zoom')).toHaveText('120%');
    await expect(page.getByLabel('Select cell-a')).toBeVisible();

    await page.getByRole('button', { name: 'Reset World Viewer navigation' }).click();
    await expect(page.getByLabel('World Viewer zoom')).toHaveText('100%');
    await expect(page.getByLabel('Select cell-a')).toBeVisible();

    await page.screenshot({ path: join(screenshotDir, '1366x768-navigation.png'), fullPage: true });
  });
});

async function openMonitor(page: Page, viewport: { width: number; height: number }) {
  await page.setViewportSize(viewport);
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'ALife Control Center' })).toBeVisible();
  await expect(page.getByLabel('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
  await expect(page.getByLabel('Viewer projection truth')).toBeVisible();
  await expect(page.getByLabel('World Viewer navigation', { exact: true })).toBeVisible();
}

async function assertWorldFirstLayout(page: Page) {
  const layers = await page.getByLabel('Layer controls').boundingBox();
  const viewer = await page.getByLabel('Monitor workspace').boundingBox();
  const world = await page.getByLabel('World Viewer', { exact: true }).boundingBox();
  const inspector = await page.getByLabel('Cell Inspector').boundingBox();
  const stats = await page.getByLabel('World stats').boundingBox();
  const focus = await page.getByLabel('Selected entity focus').boundingBox();

  expect(layers).not.toBeNull();
  expect(viewer).not.toBeNull();
  expect(world).not.toBeNull();
  expect(inspector).not.toBeNull();
  expect(stats).not.toBeNull();
  expect(focus).not.toBeNull();

  const l = layers!;
  const v = viewer!;
  const w = world!;
  const i = inspector!;
  const s = stats!;
  const f = focus!;

  expect(w.width).toBeGreaterThan(l.width);
  expect(w.width).toBeGreaterThan(i.width);
  expect(w.height).toBeGreaterThan(360);
  expect(v.x).toBeGreaterThan(l.x + l.width - 1);
  expect(i.x).toBeGreaterThan(v.x + v.width - 1);
  expect(s.y).toBeGreaterThan(w.y + w.height - 8);
  expect(f.y + f.height).toBeLessThanOrEqual(s.y + 1);
}
