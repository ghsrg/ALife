import { defineConfig, devices } from '@playwright/test';
import { existsSync } from 'node:fs';

const browserExecutablePath = [
  process.env.E2E_BROWSER_EXECUTABLE,
  'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe',
  'C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe'
].find((candidate): candidate is string => Boolean(candidate && existsSync(candidate)));

const browserLaunchOptions = browserExecutablePath ? { executablePath: browserExecutablePath } : undefined;

export default defineConfig({
  testDir: './tests/e2e',
  testIgnore: ['live-runner.spec.ts'],
  fullyParallel: false,
  reporter: [['list']],
  use: {
    baseURL: 'http://127.0.0.1:5173',
    launchOptions: browserLaunchOptions,
    trace: 'on-first-retry'
  },
  webServer: {
    command: 'npm.cmd run dev -- --host 127.0.0.1 --port 5173',
    url: 'http://127.0.0.1:5173',
    reuseExistingServer: !process.env.CI,
    timeout: 120_000
  },
  projects: [
    {
      name: 'chromium',
      use: {
        ...devices['Desktop Chrome'],
        launchOptions: browserLaunchOptions
      }
    }
  ]
});
