import { test, expect } from '@playwright/test';

test('verify chyren web app is online and connected to medulla', async ({ page }) => {
  // 1. Navigate to the app
  await page.goto('https://chyren-web.vercel.app');

  // 2. Wait for the page to load
  await expect(page).toHaveTitle(/Chyren/i);

  // 3. Verify that "Medulla Offline" is NOT present
  // We'll wait a few seconds to let the metrics fetch happen (it polls every 2s)
  await page.waitForTimeout(3000);
  const offlineError = page.locator('text=Medulla Offline');
  await expect(offlineError).not.toBeVisible();

  // 4. Verify that Metrics are showing up
  // Check for the "Task Admission" or "Active Runs" labels in the dashboard
  await expect(page.locator('text=Task Admission')).toBeVisible();
  await expect(page.locator('text=Active Runs')).toBeVisible();

  // 5. Try to interact with the chat to ensure it's not "Not Configured"
  const chatInput = page.getByPlaceholder(/Send a message/i);
  if (await chatInput.isVisible()) {
    await chatInput.fill('ping');
    await chatInput.press('Enter');
    
    // Check that we don't get the "not fully configured" error message
    const errorMessage = page.locator('text=Chyren is not fully configured yet');
    await expect(errorMessage).not.toBeVisible({ timeout: 10000 });
  }
});
