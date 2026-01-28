// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Shared Settings', () => {
  test.beforeEach(async ({ page }) => {
    // Clear localStorage before each test
    await page.goto('/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Wait for WASM to initialize
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  });

  test('should display default settings values', async ({ page }) => {
    await page.goto('/');

    // Check default values
    const opportunityRate = await page.inputValue('#opportunity-rate');
    expect(opportunityRate).toBe('8');

    const annualMileage = await page.inputValue('#annual-mileage');
    expect(annualMileage).toBe('12000');

    const lifetimeMiles = await page.inputValue('#lifetime-miles');
    expect(lifetimeMiles).toBe('200000');

    const gasPrice = await page.inputValue('#average-gas-price');
    expect(gasPrice).toBe('3.5');
  });

  test('should update opportunity cost rate', async ({ page }) => {
    await page.goto('/');

    // Change opportunity cost rate
    await page.fill('#opportunity-rate', '10');
    await page.waitForTimeout(500);

    // Verify the value was updated
    const newValue = await page.inputValue('#opportunity-rate');
    expect(newValue).toBe('10');

    // Verify it's saved to localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );
    const settings = JSON.parse(storage);
    expect(settings.opportunity_cost_rate).toBe(10);
  });

  test('should update annual mileage', async ({ page }) => {
    await page.goto('/');

    // Change annual mileage
    await page.fill('#annual-mileage', '15000');
    await page.waitForTimeout(500);

    // Verify the value was updated
    const newValue = await page.inputValue('#annual-mileage');
    expect(newValue).toBe('15000');

    // Verify it's saved to localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );
    const settings = JSON.parse(storage);
    expect(settings.annual_mileage).toBe(15000);
  });

  test('should update lifetime miles', async ({ page }) => {
    await page.goto('/');

    // Change lifetime miles
    await page.fill('#lifetime-miles', '250000');
    await page.waitForTimeout(500);

    // Verify the value was updated
    const newValue = await page.inputValue('#lifetime-miles');
    expect(newValue).toBe('250000');

    // Verify it's saved to localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );
    const settings = JSON.parse(storage);
    expect(settings.lifetime_miles).toBe(250000);
  });

  test('should update average gas price', async ({ page }) => {
    await page.goto('/');

    // Change gas price
    await page.fill('#average-gas-price', '4.25');
    await page.waitForTimeout(500);

    // Verify the value was updated
    const newValue = await page.inputValue('#average-gas-price');
    expect(newValue).toBe('4.25');

    // Verify it's saved to localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );
    const settings = JSON.parse(storage);
    expect(settings.average_gas_price).toBe(4.25);
  });

  test('should persist settings across page reloads', async ({ page }) => {
    await page.goto('/');

    // Update multiple settings
    await page.fill('#opportunity-rate', '12');
    await page.fill('#annual-mileage', '18000');
    await page.fill('#lifetime-miles', '300000');
    await page.fill('#average-gas-price', '5.00');
    await page.waitForTimeout(500);

    // Reload the page
    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // Verify all values persisted
    expect(await page.inputValue('#opportunity-rate')).toBe('12');
    expect(await page.inputValue('#annual-mileage')).toBe('18000');
    expect(await page.inputValue('#lifetime-miles')).toBe('300000');
    expect(await page.inputValue('#average-gas-price')).toBe('5');
  });

  test('should handle decimal inputs for opportunity cost rate', async ({ page }) => {
    await page.goto('/');

    await page.fill('#opportunity-rate', '7.5');
    await page.waitForTimeout(500);

    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );
    const settings = JSON.parse(storage);
    expect(settings.opportunity_cost_rate).toBe(7.5);
  });

  test('should handle decimal inputs for gas price', async ({ page }) => {
    await page.goto('/');

    await page.fill('#average-gas-price', '3.89');
    await page.waitForTimeout(500);

    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_settings')
    );
    const settings = JSON.parse(storage);
    expect(settings.average_gas_price).toBe(3.89);
  });
});
