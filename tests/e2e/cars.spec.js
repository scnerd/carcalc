// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Car Management', () => {
  test.beforeEach(async ({ page }) => {
    // Clear localStorage before each test
    await page.goto('/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Wait for WASM to initialize
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  });

  test('should display empty state when no cars exist', async ({ page }) => {
    await page.goto('/');

    // Should show empty state message
    await expect(page.getByText('No cars yet')).toBeVisible();
    await expect(page.getByText('Get started by adding a car to compare.')).toBeVisible();
  });

  test('should add a new car', async ({ page }) => {
    await page.goto('/');

    // Click Add Car button
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Should show a new car card with default name
    await expect(page.getByText(/Car #1/)).toBeVisible();

    // Empty state should be hidden
    await expect(page.getByText('No cars yet')).not.toBeVisible();
  });

  test('should expand and collapse car details', async ({ page }) => {
    await page.goto('/');

    // Add a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Car should be expanded by default (fields visible)
    const makeInput = page.getByLabel('Make').first();
    await expect(makeInput).toBeVisible();

    // Click to collapse
    await page.getByText(/Car #1/).click();
    await page.waitForTimeout(300);

    // Fields should be hidden
    await expect(makeInput).not.toBeVisible();

    // Click to expand again
    await page.getByText(/Car #1/).click();
    await page.waitForTimeout(300);

    // Fields should be visible again
    await expect(makeInput).toBeVisible();
  });

  test('should fill out car information', async ({ page }) => {
    await page.goto('/');

    // Add a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(500);

    // Fill out all required fields
    await page.getByLabel('Make').first().fill('Toyota');
    await page.getByLabel('Model').first().fill('Camry');
    await page.getByLabel('Trim/Features (optional)').first().fill('XLE');
    await page.getByLabel('Model Year').first().fill('2020');
    await page.getByLabel(/Purchase Price/).first().fill('25000');
    await page.getByLabel(/Current Mileage/).first().fill('50000');
    await page.getByLabel('MPG').first().fill('32');
    await page.getByLabel(/Insurance Cost/).first().fill('600');

    await page.waitForTimeout(500);

    // Verify the car title updates
    await expect(page.getByText('Toyota Camry (2020)')).toBeVisible();
  });

  test('should persist car data to localStorage', async ({ page }) => {
    await page.goto('/');

    // Add a car and fill it out
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel('Make').first().fill('Honda');
    await page.getByLabel('Model').first().fill('Civic');
    await page.getByLabel(/Purchase Price/).first().fill('22000');
    await page.waitForTimeout(500);

    // Check localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );
    const cars = JSON.parse(storage);

    expect(cars).toHaveLength(1);
    expect(cars[0].make).toBe('Honda');
    expect(cars[0].model).toBe('Civic');
    expect(cars[0].purchase_price).toBe('22000');
  });

  test('should add multiple cars', async ({ page }) => {
    await page.goto('/');

    // Add three cars
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Should see all three cars
    await expect(page.getByText(/Car #1/)).toBeVisible();
    await expect(page.getByText(/Car #2/)).toBeVisible();
    await expect(page.getByText(/Car #3/)).toBeVisible();

    // Check localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );
    const cars = JSON.parse(storage);
    expect(cars).toHaveLength(3);
  });

  test('should delete a car', async ({ page }) => {
    await page.goto('/');

    // Add two cars
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByLabel('Make').first().fill('Ford');
    await page.waitForTimeout(300);

    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Click the delete button on the first car
    const deleteButtons = page.locator('button[class*="text-red-600"]');
    await deleteButtons.first().click();
    await page.waitForTimeout(300);

    // First car should be gone
    await expect(page.getByText('Ford')).not.toBeVisible();

    // Second car should still exist
    await expect(page.getByText(/Car #2/)).toBeVisible();

    // Check localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );
    const cars = JSON.parse(storage);
    expect(cars).toHaveLength(1);
    expect(cars[0].id).toBe(2);
  });

  test('should fill out optional fields', async ({ page }) => {
    await page.goto('/');

    // Add a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Fill required fields
    await page.getByLabel('Make').first().fill('Tesla');
    await page.getByLabel('Model').first().fill('Model 3');
    await page.getByLabel(/Purchase Price/).first().fill('40000');
    await page.getByLabel(/Current Mileage/).first().fill('15000');
    await page.getByLabel('MPG').first().fill('120');
    await page.getByLabel(/Insurance Cost/).first().fill('800');

    // Fill optional fields
    await page.getByLabel('VIN (optional)').first().fill('5YJ3E1EA9KF123456');
    await page.getByLabel('Listing URL (optional)').first().fill('https://example.com/car');
    await page.getByLabel('Notes (optional)').first().fill('Great car, low maintenance');

    await page.waitForTimeout(500);

    // Verify in localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );
    const cars = JSON.parse(storage);

    expect(cars[0].vin).toBe('5YJ3E1EA9KF123456');
    expect(cars[0].listing_url).toBe('https://example.com/car');
    expect(cars[0].notes).toBe('Great car, low maintenance');
  });

  test('should persist cars across page reloads', async ({ page }) => {
    await page.goto('/');

    // Add and fill out a car
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel('Make').first().fill('Mazda');
    await page.getByLabel('Model').first().fill('CX-5');
    await page.getByLabel('Model Year').first().fill('2021');
    await page.getByLabel(/Purchase Price/).first().fill('30000');
    await page.waitForTimeout(500);

    // Reload the page
    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    // Car should still be there with all data
    await expect(page.getByText('Mazda CX-5 (2021)')).toBeVisible();

    // Expand and verify fields
    await page.getByText('Mazda CX-5 (2021)').click();
    await page.waitForTimeout(300);

    const makeValue = await page.getByLabel('Make').first().inputValue();
    expect(makeValue).toBe('Mazda');

    const modelValue = await page.getByLabel('Model').first().inputValue();
    expect(modelValue).toBe('CX-5');

    const yearValue = await page.getByLabel('Model Year').first().inputValue();
    expect(yearValue).toBe('2021');
  });

  test('should handle updating car information', async ({ page }) => {
    await page.goto('/');

    // Add a car with initial data
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    await page.getByLabel('Make').first().fill('BMW');
    await page.getByLabel('Model').first().fill('3 Series');
    await page.waitForTimeout(500);

    // Update the make
    await page.getByLabel('Make').first().fill('Audi');
    await page.waitForTimeout(500);

    // Verify title updated
    await expect(page.getByText('Audi 3 Series')).toBeVisible();

    // Verify in localStorage
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );
    const cars = JSON.parse(storage);
    expect(cars[0].make).toBe('Audi');
  });

  test('should maintain car IDs correctly after deletions', async ({ page }) => {
    await page.goto('/');

    // Add three cars
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Delete the second car
    const deleteButtons = page.locator('button[class*="text-red-600"]');
    await deleteButtons.nth(1).click();
    await page.waitForTimeout(300);

    // Add another car - it should get ID 4, not reuse ID 2
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Check localStorage to verify IDs
    const storage = await page.evaluate(() =>
      localStorage.getItem('carcalc_cars')
    );
    const cars = JSON.parse(storage);

    expect(cars).toHaveLength(3);
    const ids = cars.map((c) => c.id).sort();
    expect(ids).toEqual([1, 3, 4]);
  });
});
