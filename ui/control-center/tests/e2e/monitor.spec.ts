import { expect, test } from '@playwright/test';

test('UI-1A Monitor opens at 1366x862 with viewer and Inspector', async ({ page }) => {
  await page.setViewportSize({ width: 1366, height: 862 });
  await page.goto('/');

  await expect(page.getByRole('heading', { name: 'ALife Control Center' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Monitor' })).toHaveAttribute('aria-selected', 'true');
  await expect(page.getByLabel('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');
  await expect(page.getByLabel('Cell Inspector')).toBeVisible();

  const lastSelectableCell = page.getByRole('button', { name: /^Select / }).last();
  await lastSelectableCell.click();
  await expect(page.getByLabel('Cell Inspector')).not.toContainText('No cell selected.');

  await page.getByRole('button', { name: 'Switch to light theme' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');

  await page.getByRole('button', { name: 'Export viewer PNG' }).click();
  await expect(page.getByRole('status')).toContainText('Start screenshot PNG ready');
});

test('AL-007-S22 keeps Monitor tracks stable at 1920x1080', async ({ page }) => {
  await page.setViewportSize({ width: 1920, height: 1080 });
  await page.goto('/');

  await expect(page.getByLabel('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');

  const navigation = await page.getByTestId('monitor-navigation-track').boundingBox();
  const run = await page.getByTestId('monitor-run-track').boundingBox();
  const level = await page.getByTestId('monitor-level-track').boundingBox();
  const layers = await page.getByTestId('monitor-layers-track').boundingBox();
  const map = await page.getByTestId('monitor-map-track').boundingBox();
  const inspector = await page.getByTestId('monitor-inspector-track').boundingBox();
  const data = await page.getByTestId('monitor-data-track').boundingBox();

  expect(navigation).not.toBeNull();
  expect(run).not.toBeNull();
  expect(level).not.toBeNull();
  expect(layers).not.toBeNull();
  expect(map).not.toBeNull();
  expect(inspector).not.toBeNull();
  expect(data).not.toBeNull();

  expect(Math.round(navigation!.height)).toBe(62);
  expect(Math.round(run!.height)).toBe(82);
  expect(Math.round(level!.width)).toBe(83);
  expect(Math.round(layers!.width)).toBe(262);
  expect(Math.round(inspector!.width)).toBe(335);
  expect(Math.round(data!.height)).toBe(281);
  expect(map!.width).toBeGreaterThan(0);
  expect(map!.height).toBeGreaterThan(600);
  expect((await page.getByLabel('World Viewer', { exact: true }).boundingBox())!.height).toBeGreaterThan(560);
  await expect(page.getByTitle('Collapse Layers')).toHaveCount(0);
  await expect(page.getByRole('navigation', { name: 'Data panel tabs' })).toHaveCount(0);
  await expect(page.getByTestId('bottom-stats-strip')).toHaveCount(0);
});

test('AL-007-S22 supports 1280x720 CSS viewport for 150 percent display scale', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto('/');

  await expect(page.getByLabel('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');

  const navigation = await page.getByTestId('monitor-navigation-track').boundingBox();
  const run = await page.getByTestId('monitor-run-track').boundingBox();
  const level = await page.getByTestId('monitor-level-track').boundingBox();
  const layers = await page.getByTestId('monitor-layers-track').boundingBox();
  const map = await page.getByTestId('monitor-map-track').boundingBox();
  const world = await page.getByLabel('World Viewer', { exact: true }).boundingBox();
  const inspector = await page.getByTestId('monitor-inspector-track').boundingBox();
  const data = await page.getByTestId('monitor-data-track').boundingBox();

  expect(navigation).not.toBeNull();
  expect(run).not.toBeNull();
  expect(level).not.toBeNull();
  expect(layers).not.toBeNull();
  expect(map).not.toBeNull();
  expect(world).not.toBeNull();
  expect(inspector).not.toBeNull();
  expect(data).not.toBeNull();

  expect(Math.round(navigation!.height)).toBe(41);
  expect(Math.round(run!.height)).toBe(55);
  expect(Math.round(level!.width)).toBe(55);
  expect(Math.round(layers!.width)).toBe(175);
  expect(Math.round(inspector!.width)).toBe(223);
  expect(Math.round(data!.height)).toBe(187);
  expect(map!.width).toBeGreaterThan(820);
  expect(world!.height).toBeGreaterThan(400);

  const pageGeometry = await page.evaluate(() => ({
    clientWidth: document.documentElement.clientWidth,
    scrollWidth: document.documentElement.scrollWidth,
    clientHeight: document.documentElement.clientHeight,
    scrollHeight: document.documentElement.scrollHeight
  }));
  expect(pageGeometry.scrollWidth).toBe(pageGeometry.clientWidth);
  expect(pageGeometry.scrollHeight).toBe(pageGeometry.clientHeight);
});

test('AL-007-S22 uses root scroll below 1280x720 without collapsing fixed tracks', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 719 });
  await page.goto('/');

  await expect(page.getByLabel('World Viewer', { exact: true })).toHaveAttribute('data-ready', 'true');

  const geometry = await page.evaluate(() => ({
    scrollHeight: document.documentElement.scrollHeight,
    clientHeight: document.documentElement.clientHeight,
    dataOverflowY: getComputedStyle(document.querySelector('[data-testid="monitor-data-track"]')!).overflowY
  }));

  expect(geometry.scrollHeight).toBeGreaterThan(geometry.clientHeight);
  expect(geometry.dataOverflowY).not.toBe('auto');
  expect((await page.getByTestId('monitor-inspector-track').boundingBox())!.width).toBeGreaterThan(220);
  expect((await page.getByTestId('monitor-map-track').boundingBox())!.width).toBeGreaterThan(0);
  expect((await page.getByTestId('monitor-map-track').boundingBox())!.height).toBeGreaterThan(400);
  expect((await page.getByLabel('World Viewer', { exact: true }).boundingBox())!.height).toBeGreaterThan(380);
});
