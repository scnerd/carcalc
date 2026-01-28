// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Cost Calculations', () => {
  test.beforeEach(async ({ page }) => {
    // Clear localStorage before each test
    await page.goto('/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Wait for WASM to initialize
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  });

  test('should display calculated costs for a complete car', async ({ page }) => {
    await page.goto('/');

    // Add a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Fill out all required fields
    await page.getByLabel('Make').first().fill('Toyota');
    await page.getByLabel('Model').first().fill('Corolla');
    await page.getByLabel(/Purchase Price/).first().fill('25000');
    await page.getByLabel(/Current Mileage/).first().fill('30000');
    await page.getByLabel('MPG').first().fill('35');
    await page.getByLabel(/Insurance Cost/).first().fill('500');

    await page.waitForTimeout(500);

    // Should display calculated costs section
    await expect(page.getByText('Calculated Costs')).toBeVisible();
    await expect(page.getByText('Total Cost of Ownership')).toBeVisible();
    await expect(page.getByText('Annual Cost')).toBeVisible();
  });

  test('should calculate remaining miles correctly', async ({ page }) => {
    await page.goto('/');

    // Set lifetime miles to 200,000 (default)
    // Add a car with 50,000 current miles
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('20000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('30');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Should show 150,000 remaining miles (200,000 - 50,000)
    const remainingMilesText = await page
      .locator('text=Remaining Miles')
      .locator('..')
      .locator('div:has-text(/^\\d+$/)')
      .first()
      .textContent();

    expect(remainingMilesText.trim()).toBe('150000');
  });

  test('should calculate years remaining correctly', async ({ page }) => {
    await page.goto('/');

    // Default annual mileage is 12,000
    // With 120,000 remaining miles, should be 10 years
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('30000');
    await page.getByLabel(/Current Mileage/).first().fill('80000'); // 200k - 80k = 120k remaining
    await page.getByLabel('MPG').first().fill('25');
    await page.getByLabel(/Insurance Cost/).first().fill('700');

    await page.waitForTimeout(500);

    // Should show 10.0 years (120,000 / 12,000)
    const yearsText = await page
      .locator('text=Years Remaining')
      .locator('..')
      .locator('div:has-text(/^\\d+\\.\\d$/)')
      .first()
      .textContent();

    expect(yearsText.trim()).toBe('10.0');
  });

  test('should calculate fuel cost based on MPG and gas price', async ({ page }) => {
    await page.goto('/');

    // Set gas price to $4.00
    await page.fill('#average-gas-price', '4.00');
    await page.waitForTimeout(300);

    // Add car with:
    // - 150,000 remaining miles (200k - 50k)
    // - 30 MPG
    // Expected fuel cost: (150,000 / 30) * 4.00 = 5,000 * 4 = $20,000
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('25000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('30');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Check total fuel cost
    const fuelCostText = await page
      .locator('text=Fuel Cost (Total)')
      .locator('..')
      .locator('div:has-text(/^\\$\\d+\\.\\d{2}$/)')
      .first()
      .textContent();

    expect(fuelCostText.trim()).toBe('$20000.00');
  });

  test('should calculate annual fuel cost correctly', async ({ page }) => {
    await page.goto('/');

    // Default: 12,000 miles/year, $3.50/gallon
    // Car with 30 MPG
    // Annual fuel: (12,000 / 30) * 3.50 = 400 * 3.50 = $1,400
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('20000');
    await page.getByLabel(/Current Mileage/).first().fill('30000');
    await page.getByLabel('MPG').first().fill('30');
    await page.getByLabel(/Insurance Cost/).first().fill('500');

    await page.waitForTimeout(500);

    // Check annual fuel cost
    const annualFuelText = await page
      .locator('text=Fuel Cost (Annual)')
      .locator('..')
      .locator('div:has-text(/^\\$\\d+\\.\\d{2}$/)')
      .first()
      .textContent();

    expect(annualFuelText.trim()).toBe('$1400.00');
  });

  test('should calculate insurance cost annually', async ({ page }) => {
    await page.goto('/');

    // 6-month premium of $600 should be $1,200/year
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('25000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('28');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Check annual insurance cost
    const insuranceText = await page
      .locator('text=Insurance (Annual)')
      .locator('..')
      .locator('div:has-text(/^\\$\\d+\\.\\d{2}$/)')
      .first()
      .textContent();

    expect(insuranceText.trim()).toBe('$1200.00');
  });

  test('should calculate opportunity cost based on purchase price and rate', async ({ page }) => {
    await page.goto('/');

    // Set opportunity cost rate to 10%
    await page.fill('#opportunity-rate', '10');
    await page.waitForTimeout(300);

    // Add car with:
    // - $30,000 purchase price
    // - 10.0 years remaining (120k miles / 12k annual)
    // Expected opportunity: 30,000 * 0.10 * 10 = $30,000
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('30000');
    await page.getByLabel(/Current Mileage/).first().fill('80000'); // 120k remaining = 10 years
    await page.getByLabel('MPG').first().fill('25');
    await page.getByLabel(/Insurance Cost/).first().fill('500');

    await page.waitForTimeout(500);

    // Check opportunity cost
    const opportunityText = await page
      .locator('text=Opportunity Cost')
      .locator('..')
      .locator('div:has-text(/^\\$\\d+\\.\\d{2}$/)')
      .first()
      .textContent();

    expect(opportunityText.trim()).toBe('$30000.00');
  });

  test('should update calculations when settings change', async ({ page }) => {
    await page.goto('/');

    // Add a car first
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel(/Purchase Price/).first().fill('25000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('30');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Get initial fuel cost
    const initialFuelText = await page
      .locator('text=Fuel Cost (Total)')
      .locator('..')
      .locator('div:has-text(/^\\$\\d+\\.\\d{2}$/)')
      .first()
      .textContent();

    // Change gas price from $3.50 to $5.00
    await page.fill('#average-gas-price', '5.00');
    await page.waitForTimeout(500);

    // Get new fuel cost - should be higher
    const newFuelText = await page
      .locator('text=Fuel Cost (Total)')
      .locator('..')
      .locator('div:has-text(/^\\$\\d+\\.\\d{2}$/)')
      .first()
      .textContent();

    // Parse and compare
    const initialCost = parseFloat(initialFuelText.replace(/[$,]/g, ''));
    const newCost = parseFloat(newFuelText.replace(/[$,]/g, ''));

    expect(newCost).toBeGreaterThan(initialCost);
  });

  test('should show total cost of ownership as sum of all costs', async ({ page }) => {
    await page.goto('/');

    // Set known values for easier calculation verification
    await page.fill('#opportunity-rate', '0'); // Disable opportunity cost for simplicity
    await page.fill('#annual-mileage', '10000');
    await page.fill('#lifetime-miles', '100000');
    await page.fill('#average-gas-price', '4.00');
    await page.waitForTimeout(300);

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Car with:
    // - Current: 50,000 miles, Remaining: 50,000 miles = 5 years
    // - Fuel: (50,000 / 25) * 4 = 2,000 * 4 = $8,000
    // - Insurance: $600 * 2 * 5 years = $6,000
    // - Opportunity: $0 (rate set to 0)
    // - Total (without maintenance): $14,000
    await page.getByLabel(/Purchase Price/).first().fill('20000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('25');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Check that total cost is displayed
    const totalCostText = await page
      .locator('text=Total Cost of Ownership')
      .locator('..')
      .locator('div.text-2xl')
      .first()
      .textContent();

    // Should be at least $14,000 (could be more with maintenance)
    const totalCost = parseFloat(totalCostText.replace(/[$,]/g, ''));
    expect(totalCost).toBeGreaterThanOrEqual(14000);
  });

  test('should display all cost breakdown fields', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Fill minimum required fields
    await page.getByLabel(/Purchase Price/).first().fill('20000');
    await page.getByLabel(/Current Mileage/).first().fill('60000');
    await page.getByLabel('MPG').first().fill('28');
    await page.getByLabel(/Insurance Cost/).first().fill('550');

    await page.waitForTimeout(500);

    // Verify all cost fields are visible
    await expect(page.getByText('Total Cost of Ownership')).toBeVisible();
    await expect(page.getByText('Annual Cost')).toBeVisible();
    await expect(page.getByText('Years Remaining')).toBeVisible();
    await expect(page.getByText('Remaining Miles')).toBeVisible();
    await expect(page.getByText('Fuel Cost (Total)')).toBeVisible();
    await expect(page.getByText('Fuel Cost (Annual)')).toBeVisible();
    await expect(page.getByText('Insurance (Annual)')).toBeVisible();
    await expect(page.getByText('Opportunity Cost')).toBeVisible();
    await expect(page.getByText('Maintenance (Total)')).toBeVisible();
    await expect(page.getByText('Maintenance (Annual)')).toBeVisible();
  });

  test('should handle zero MPG gracefully', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Enter 0 for MPG (invalid but shouldn't crash)
    await page.getByLabel(/Purchase Price/).first().fill('25000');
    await page.getByLabel(/Current Mileage/).first().fill('40000');
    await page.getByLabel('MPG').first().fill('0');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Should still display costs section without errors
    await expect(page.getByText('Calculated Costs')).toBeVisible();
  });

  test('should recalculate when car mileage changes', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Initial state: 50,000 current miles
    await page.getByLabel(/Purchase Price/).first().fill('22000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('30');
    await page.getByLabel(/Insurance Cost/).first().fill('500');

    await page.waitForTimeout(500);

    // Get initial remaining miles
    const initialRemainingText = await page
      .locator('text=Remaining Miles')
      .locator('..')
      .locator('div:has-text(/^\\d+$/)')
      .first()
      .textContent();

    // Change current mileage to 100,000
    await page.getByLabel(/Current Mileage/).first().fill('100000');
    await page.waitForTimeout(500);

    // Get new remaining miles - should be less
    const newRemainingText = await page
      .locator('text=Remaining Miles')
      .locator('..')
      .locator('div:has-text(/^\\d+$/)')
      .first()
      .textContent();

    const initialRemaining = parseInt(initialRemainingText.trim());
    const newRemaining = parseInt(newRemainingText.trim());

    expect(newRemaining).toBeLessThan(initialRemaining);
    expect(newRemaining).toBe(100000); // 200,000 - 100,000
  });
});
