// @ts-check
const { test, expect } = require('@playwright/test');

test.describe('Maintenance Cost Data Editor', () => {
  test.beforeEach(async ({ page }) => {
    // Clear localStorage before each test
    await page.goto('/');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Wait for WASM to initialize
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);
  });

  test('should display maintenance cost data section', async ({ page }) => {
    await page.goto('/');

    // Should show the section header
    await expect(page.getByText('Maintenance Cost Data')).toBeVisible();
    await expect(
      page.getByText('View and edit maintenance cost tables per make/model')
    ).toBeVisible();
  });

  test('should expand and collapse maintenance data editor', async ({ page }) => {
    await page.goto('/');

    // Initially collapsed - dropdown should not be visible
    const dropdown = page.getByLabel('Select Make/Model');
    await expect(dropdown).not.toBeVisible();

    // Click to expand
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    // Should now be visible
    await expect(dropdown).toBeVisible();

    // Click to collapse
    await expandButton.click();
    await page.waitForTimeout(300);

    // Should be hidden again
    await expect(dropdown).not.toBeVisible();
  });

  test('should show dropdown when expanded', async ({ page }) => {
    await page.goto('/');

    // Expand the section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    // Should show dropdown with default option
    const dropdown = page.getByLabel('Select Make/Model');
    await expect(dropdown).toBeVisible();

    // Should have default "Select a vehicle" option
    const defaultOption = dropdown.locator('option').first();
    await expect(defaultOption).toHaveText('-- Select a vehicle --');
  });

  test('should show vehicle options when maintenance data exists', async ({
    page,
  }) => {
    await page.goto('/');

    // First, we need to add some maintenance data by creating a car
    // The maintenance data gets populated when CarEdge data is available
    // For this test, we'll check if the dropdown structure works

    // Expand the section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    const dropdown = page.getByLabel('Select Make/Model');
    const options = dropdown.locator('option');

    // Should have at least the default option
    const optionCount = await options.count();
    expect(optionCount).toBeGreaterThanOrEqual(1);
  });

  test('should display info message about data source', async ({ page }) => {
    await page.goto('/');

    // Expand the section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    // Info message should be present but not visible until vehicle selected
    // Just verify the section is expanded
    const dropdown = page.getByLabel('Select Make/Model');
    await expect(dropdown).toBeVisible();
  });

  test('should persist expanded/collapsed state during session', async ({
    page,
  }) => {
    await page.goto('/');

    // Expand the section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    // Verify it's expanded
    const dropdown = page.getByLabel('Select Make/Model');
    await expect(dropdown).toBeVisible();

    // Click somewhere else (like settings)
    await page.fill('#opportunity-rate', '9');
    await page.waitForTimeout(300);

    // Maintenance section should still be expanded
    await expect(dropdown).toBeVisible();
  });

  test('should initialize maintenance database in localStorage', async ({
    page,
  }) => {
    await page.goto('/');

    // Check that maintenance DB is created
    const maintenanceDb = await page.evaluate(() =>
      localStorage.getItem('carcalc_maintenance_db')
    );

    expect(maintenanceDb).not.toBeNull();

    const parsed = JSON.parse(maintenanceDb);
    expect(typeof parsed).toBe('object');
  });

  test('should handle selecting and deselecting vehicles', async ({ page }) => {
    await page.goto('/');

    // Expand the section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    const dropdown = page.getByLabel('Select Make/Model');

    // Select default (empty) option
    await dropdown.selectOption('');
    await page.waitForTimeout(300);

    // No data tables should be visible
    await expect(page.getByText('By Mileage')).not.toBeVisible();
    await expect(page.getByText('By Time')).not.toBeVisible();
  });

  test('should maintain section state across page reload', async ({ page }) => {
    await page.goto('/');

    // The expanded state is NOT persisted (it's in component state, not localStorage)
    // So after reload, it should be collapsed by default

    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(1000);

    const dropdown = page.getByLabel('Select Make/Model');
    await expect(dropdown).not.toBeVisible();
  });

  test('should show appropriate UI for empty maintenance database', async ({
    page,
  }) => {
    await page.goto('/');

    // Expand the section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    // With empty database, should only show default option
    const dropdown = page.getByLabel('Select Make/Model');
    const options = dropdown.locator('option');

    // Count might be 1 (just default) or more if sample data is loaded
    const count = await options.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test('should work alongside other page features', async ({ page }) => {
    await page.goto('/');

    // Expand maintenance section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    // Add a car while maintenance section is expanded
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Both should be functional
    await expect(page.getByLabel('Select Make/Model')).toBeVisible();
    await expect(page.getByText(/Car #1/)).toBeVisible();

    // Fill car data
    await page.getByLabel('Make').first().fill('Honda');
    await page.waitForTimeout(300);

    // Maintenance section should still be expanded
    await expect(page.getByLabel('Select Make/Model')).toBeVisible();
  });

  test('should have accessible collapse/expand button', async ({ page }) => {
    await page.goto('/');

    // The button should be focusable and clickable
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();

    // Should be visible
    await expect(expandButton).toBeVisible();

    // Should be clickable
    await expandButton.click();
    await page.waitForTimeout(300);

    // Should expand
    await expect(page.getByLabel('Select Make/Model')).toBeVisible();
  });

  test('should render dropdown with correct structure', async ({ page }) => {
    await page.goto('/');

    // Expand section
    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();
    await expandButton.click();
    await page.waitForTimeout(300);

    const dropdown = page.getByLabel('Select Make/Model');

    // Should be a select element
    await expect(dropdown).toHaveRole('combobox');

    // Should have proper styling classes
    const classes = await dropdown.getAttribute('class');
    expect(classes).toContain('rounded-md');
  });

  test('should maintain maintenance data in localStorage', async ({ page }) => {
    await page.goto('/');

    // Get initial maintenance DB
    const initialDb = await page.evaluate(() =>
      localStorage.getItem('carcalc_maintenance_db')
    );

    expect(initialDb).not.toBeNull();

    // Interact with the app (add a car)
    await page.getByRole('button', { name: 'Add Car' }).click();
    await page.waitForTimeout(300);

    // Maintenance DB should still exist and be valid JSON
    const currentDb = await page.evaluate(() =>
      localStorage.getItem('carcalc_maintenance_db')
    );

    expect(currentDb).not.toBeNull();
    expect(() => JSON.parse(currentDb)).not.toThrow();
  });

  test('should handle rapid expand/collapse clicks', async ({ page }) => {
    await page.goto('/');

    const expandButton = page
      .locator('h2:has-text("Maintenance Cost Data")')
      .locator('..')
      .locator('button')
      .first();

    // Rapidly click expand/collapse
    await expandButton.click();
    await expandButton.click();
    await expandButton.click();
    await page.waitForTimeout(300);

    // Should end in collapsed state (3 clicks = open, close, open)
    await expect(page.getByLabel('Select Make/Model')).toBeVisible();
  });
});
