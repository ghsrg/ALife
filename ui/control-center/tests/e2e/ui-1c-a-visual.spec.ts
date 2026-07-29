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
    await assertWorldFirstLayout(page, { width: 1920, height: 1080 });
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Resources');
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Fixture grid');
    await expect(page.getByLabel('Viewer projection truth')).toContainText('Cell size');
    await page.screenshot({ path: join(screenshotDir, '1920x1080-dark.png'), fullPage: true });
  });

  test('1280x720 dark keeps controls usable at 150 percent display scale baseline', async ({ page }) => {
    await openMonitor(page, { width: 1280, height: 720 });

    await assertWorldFirstLayout(page, { width: 1280, height: 720 });
    await expect(page.getByRole('button', { name: 'Play live run' })).toBeVisible();
    await expect(page.getByTestId('monitor-data-track')).toBeVisible();
    await expect(page.getByLabel('Debug Visualization Mode')).toHaveClass(/collapsed/);
    await page.screenshot({ path: join(screenshotDir, '1280x720-dark.png'), fullPage: true });
  });

  test('1920x1080 light remains usable', async ({ page }) => {
    await openMonitor(page, { width: 1920, height: 1080 });

    await page.getByRole('button', { name: 'Switch to light theme' }).click();

    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
    await assertWorldFirstLayout(page, { width: 1920, height: 1080 });
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

  test('1366x862 viewer navigation zooms and keeps cell targets usable', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 862 });

    await expect(page.getByLabel('World Viewer zoom')).toHaveText('1:2600');
    await page.getByRole('button', { name: 'Zoom in World Viewer' }).click();
    await expect(page.getByLabel('World Viewer zoom')).not.toHaveText('1:2600');
    await expect(page.getByRole('button', { name: /^Select / }).first()).toBeVisible();

    for (let i = 0; i < 20; i += 1) {
      await page.getByRole('button', { name: 'Zoom in World Viewer' }).click();
    }
    await expect(page.getByRole('button', { name: /^Select / }).first()).toBeVisible();

    await page.screenshot({ path: join(screenshotDir, '1366x862-navigation.png'), fullPage: true });
  });

  test('viewer wheel and projection notices stay isolated from page gestures', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 862 });

    await page.evaluate(() => window.scrollTo(0, 160));
    const viewer = page.getByLabel('World Viewer', { exact: true });
    await viewer.hover();
    const beforeScroll = await page.evaluate(() => window.scrollY);
    await page.mouse.wheel(0, -600);

    await expect(page.getByLabel('World Viewer zoom')).not.toHaveText('1:2600');
    expect(await page.evaluate(() => window.scrollY)).toBe(beforeScroll);

    await page.getByRole('button', { name: 'Dismiss projection notices' }).click();
    await expect(page.getByLabel('Viewer projection truth')).toBeHidden();
  });

  test('cell selection remains available after dragging the viewer map', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 862 });

    const viewerBox = await page.getByLabel('World Viewer', { exact: true }).boundingBox();
    expect(viewerBox).not.toBeNull();
    const box = viewerBox!;

    await page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.5);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width * 0.5 + 42, box.y + box.height * 0.5 - 24);
    await page.mouse.up();

    await page.getByLabel('Select cell-c').click();

    await expect(page.getByLabel('Cell Inspector')).toContainText('cell-c');
  });

  test('empty viewer click clears selected Cell panels without breaking reselection', async ({ page }) => {
    await openMonitor(page, { width: 1366, height: 862 });

    await expect(page.getByLabel('Selected entity focus')).toBeVisible();
    await page.getByLabel('World Viewer', { exact: true }).click({ position: { x: 24, y: 160 } });

    await expect(page.getByLabel('Selected entity focus')).toBeHidden();
    await expect(page.getByLabel('Cell Inspector')).toContainText('No cell selected.');

    await page.getByLabel('Select cell-c').click();

    await expect(page.getByLabel('Selected entity focus')).toContainText('Cell cell-c');
    await expect(page.getByLabel('Cell Inspector')).toContainText('cell-c');
  });
});

async function openMonitor(page: Page, viewport: { width: number; height: number }) {
  await page.setViewportSize(viewport);
  await page.goto('/');
  await expect(page.getByRole('heading', { name: 'ALife Control Center' })).toBeVisible();
  await expect(page.getByLabel('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
  await expect(page.getByLabel('World Viewer zoom')).toHaveText('1:2600');
  await expect(page.getByLabel('Viewer projection truth')).toBeVisible();
  await expect(page.getByLabel('World Viewer navigation', { exact: true })).toBeVisible();
}

async function assertWorldFirstLayout(page: Page, viewport: { width: number; height: number }) {
  const layers = await page.getByLabel('Layer controls').boundingBox();
  const viewer = await page.getByLabel('Monitor workspace').boundingBox();
  const world = await page.getByLabel('World Viewer', { exact: true }).boundingBox();
  const inspector = await page.getByLabel('Cell Inspector').boundingBox();
  const data = await page.getByTestId('monitor-data-track').boundingBox();
  const focus = await page.getByLabel('Selected entity focus').boundingBox();

  expect(layers).not.toBeNull();
  expect(viewer).not.toBeNull();
  expect(world).not.toBeNull();
  expect(inspector).not.toBeNull();
  expect(data).not.toBeNull();
  expect(focus).not.toBeNull();

  const l = layers!;
  const v = viewer!;
  const w = world!;
  const i = inspector!;
  const d = data!;
  const f = focus!;

  expect(w.width).toBeGreaterThan(l.width);
  expect(w.width).toBeGreaterThan(i.width);
  expect(w.height).toBeGreaterThan(380);
  expect(w.width * w.height).toBeGreaterThan(l.width * l.height);
  expect(w.width * w.height).toBeGreaterThan(i.width * i.height);
  expect(v.x).toBeGreaterThan(l.x + l.width - 1);
  expect(i.x).toBeGreaterThan(v.x + v.width - 1);
  expect(f.x).toBeGreaterThanOrEqual(v.x);
  expect(f.x + f.width).toBeLessThanOrEqual(v.x + v.width + 1);
  expect(f.y).toBeGreaterThanOrEqual(v.y);
  expect(f.y).toBeLessThan(w.y + 80);
  expect(d.y).toBeGreaterThan(v.y + v.height - 1);
  expect(Math.round(d.height)).toBe(expectedDataPanelHeight(viewport.height));
}

function expectedDataPanelHeight(viewportHeight: number) {
  return Math.round(Math.min(281, Math.max(187, viewportHeight * (281 / 1080))));
}
