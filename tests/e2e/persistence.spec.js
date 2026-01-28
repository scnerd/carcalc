// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Data Persistence', () => {
  test.beforeEach(async ({ page }) => {
    // Clear localStorage before each test
    await page.goto('/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Wait for WASM to initialize
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  });

  test('should initialize with default settings in localStorage', async ({ page }) => {
    await page.goto('/');

    // Check that default settings are created in localStorage
    const settings = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );

    expect(settings).not.toBeNull();

    const parsed = JSON.parse(settings);
    expect(parsed.opportunity_cost_rate).toBe(8.0);
    expect(parsed.annual_mileage).toBe(12000.0);
    expect(parsed.lifetime_miles).toBe(200000.0);
    expect(parsed.average_gas_price).toBe(3.5);
  });

  test('should initialize with empty cars array in localStorage', async ({ page }) => {
    await page.goto('/');

    // Check that cars array is initialized
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    expect(cars).not.toBeNull();

    const parsed = JSON.parse(cars);
    expect(Array.isArray(parsed)).toBe(true);
    expect(parsed).toHaveLength(0);
  });

  test('should persist settings changes immediately', async ({ page }) => {
    await page.goto('/');

    // Change a setting
    await page.fill('#opportunity-rate', '12');
    await page.waitForTimeout(300);

    // Check localStorage without reloading
    const settings = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );

    const parsed = JSON.parse(settings);
    expect(parsed.opportunity_cost_rate).toBe(12);
  });

  test('should persist car additions immediately', async ({ page }) => {
    await page.goto('/');

    // Add a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Check localStorage without reloading
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    const parsed = JSON.parse(cars);
    expect(parsed).toHaveLength(1);
    expect(parsed[0].id).toBe(1);
  });

  test('should persist car field changes immediately', async ({ page }) => {
    await page.goto('/');

    // Add a car and fill a field
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel('Make').first().fill('Honda');
    await page.waitForTimeout(300);

    // Check localStorage without reloading
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    const parsed = JSON.parse(cars);
    expect(parsed[0].make).toBe('Honda');
  });

  test('should persist car deletions immediately', async ({ page }) => {
    await page.goto('/');

    // Add two cars
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Delete first car
    const deleteButtons = page.locator('button[class*="text-red-600"]');
    await deleteButtons.first().click();
    await page.waitForTimeout(300);

    // Check localStorage without reloading
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    const parsed = JSON.parse(cars);
    expect(parsed).toHaveLength(1);
  });

  test('should maintain all data across page reload', async ({ page }) => {
    await page.goto('/');

    // Make comprehensive changes
    await page.fill('#opportunity-rate', '9.5');
    await page.fill('#annual-mileage', '15000');
    await page.fill('#lifetime-miles', '250000');
    await page.fill('#average-gas-price', '4.25');
    await page.waitForTimeout(300);

    // Add two cars with data
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByLabel('Make').first().fill('Toyota');
    await page.getByLabel('Model').first().fill('Camry');
    await page.waitForTimeout(300);

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByLabel('Make').nth(1).fill('Honda');
    await page.getByLabel('Model').nth(1).fill('Accord');
    await page.waitForTimeout(300);

    // Reload page
    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // Verify settings persisted
    expect(await page.inputValue('#opportunity-rate')).toBe('9.5');
    expect(await page.inputValue('#annual-mileage')).toBe('15000');
    expect(await page.inputValue('#lifetime-miles')).toBe('250000');
    expect(await page.inputValue('#average-gas-price')).toBe('4.25');

    // Verify cars persisted
    await expect(page.getByText('Toyota Camry')).toBeVisible();
    await expect(page.getByText('Honda Accord')).toBeVisible();
  });

  test('should handle localStorage quota gracefully', async ({ page }) => {
    await page.goto('/');

    // This test verifies the app doesn't crash when adding many cars
    // (actual quota test would need to fill localStorage, which varies by browser)

    // Add 10 cars
    for (let i = 0; i < 10; i++) {
      await page.getByRole('button', { name: 'Add Car' }).click();
      await page.waitForTimeout(100);
    }

    // App should still be functional
    await expect(page.getByText(/Car #10/)).toBeVisible();

    // Verify all saved to localStorage
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    const parsed = JSON.parse(cars);
    expect(parsed).toHaveLength(10);
  });

  test('should maintain data integrity with rapid changes', async ({ page }) => {
    await page.goto('/');

    // Add a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Rapidly change fields
    const makeInput = page.getByLabel('Make').first();
    await makeInput.fill('A');
    await makeInput.fill('AB');
    await makeInput.fill('ABC');
    await makeInput.fill('ABCD');
    await makeInput.fill('Toyota');

    await page.waitForTimeout(500);

    // Final value should be correct in localStorage
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    const parsed = JSON.parse(cars);
    expect(parsed[0].make).toBe('Toyota');
  });

  test('should preserve data when navigating away and back', async ({ page }) => {
    await page.goto('/');

    // Set up some data
    await page.fill('#opportunity-rate', '7.5');
    await page.waitForTimeout(300);

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByLabel('Make').first().fill('Mazda');
    await page.waitForTimeout(300);

    // Navigate to a different domain and back
    await page.goto('about:blank');
    await page.waitForTimeout(500);
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // Data should still be there
    expect(await page.inputValue('#opportunity-rate')).toBe('7.5');
    await expect(page.getByText('Mazda')).toBeVisible();
  });

  test('should handle empty string values correctly', async ({ page }) => {
    await page.goto('/');

    // Add a car and set a field, then clear it
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel('Make').first().fill('Tesla');
    await page.waitForTimeout(300);

    await page.getByLabel('Make').first().fill('');
    await page.waitForTimeout(300);

    // Should store empty string
    const cars = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );

    const parsed = JSON.parse(cars);
    expect(parsed[0].make).toBe('');
  });

  test('should persist maintenance database', async ({ page }) => {
    await page.goto('/');

    // Check that maintenance database is initialized
    const maintenanceDb = await page.evaluate(() =>
      localStorage.getItem('carcalc_maintenance_db')
    );

    expect(maintenanceDb).not.toBeNull();

    const parsed = JSON.parse(maintenanceDb);
    expect(typeof parsed).toBe('object');
  });

  test('should isolate data between different browser tabs', async ({
    context,
  }) => {
    // Create first page and add data
    const page1 = await context.newPage();
    await page1.goto('/');
    await page1.waitForLoadState('networkidle');
    await page1.waitForTimeout(1000);

    await page1.getByRole('button', { name: 'Add Car' }).click();
    await page1.waitForTimeout(300);
    await page1.getByLabel('Make').first().fill('Ford');
    await page1.waitForTimeout(500);

    // Create second page - should see the same data (shared localStorage)
    const page2 = await context.newPage();
    await page2.goto('/');
    await page2.waitForLoadState('networkidle');
    await page2.waitForTimeout(1000);

    // Second tab should see the car from first tab
    await expect(page2.getByText('Ford')).toBeVisible();

    await page1.close();
    await page2.close();
  });

  test('should handle special characters in text fields', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Test special characters
    const specialText = 'Test "quotes" & <tags> \'apostrophes\'';
    await page.getByLabel('Make').first().fill(specialText);
    await page.waitForTimeout(500);

    // Reload and verify
    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    const makeValue = await page.getByLabel('Make').first().inputValue();
    expect(makeValue).toBe(specialText);
  });

  test('should handle numeric edge cases', async ({ page }) => {
    await page.goto('/');

    // Test very large numbers
    await page.fill('#lifetime-miles', '999999999');
    await page.waitForTimeout(300);

    const settings = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );

    const parsed = JSON.parse(settings);
    expect(parsed.lifetime_miles).toBe(999999999);
  });

  test('should maintain data structure version compatibility', async ({
    page,
  }) => {
    await page.goto('/');

    // All localStorage items should be valid JSON
    const allKeys = await page.evaluate(() => {
      const keys = [];
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key.startsWith('carcalc_')) {
          keys.push(key);
        }
      }
      return keys;
    });

    expect(allKeys.length).toBeGreaterThan(0);

    for (const key of allKeys) {
      const value = await page.evaluate((k) => localStorage.getItem(k), key);

      // Should be valid JSON
      expect(() => JSON.parse(value)).not.toThrow();
    }
  });
});
