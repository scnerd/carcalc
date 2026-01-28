# Future Features & Enhancements

This document captures potential features and improvements for CarCalc, organized by priority and theme.

## High Priority - Core Functionality

### Multi-Car Comparison View
**Impact**: High | **Effort**: Medium

Create a side-by-side comparison table showing all cars with key metrics:
- Sortable columns (TCO, annual cost, years remaining, etc.)
- Highlight best/worst values in each category
- Sticky header for scrolling
- Export comparison as CSV or print view

**Why**: This is the killer feature that makes the app truly valuable. Without it, users must mentally compare cars.

**Implementation**: New `ComparisonTable` component in Phase 7.

### Visual Cost Breakdown Charts
**Impact**: High | **Effort**: Medium

Add bar charts and pie charts showing cost composition:
- Stacked bar chart showing TCO breakdown by category
- Pie chart showing percentage of each cost type
- Side-by-side bars comparing multiple cars
- Interactive tooltips with exact values

**Why**: Visual representation makes cost differences immediately obvious.

**Implementation**: Add charting library (chart.js or similar), create `CostCharts` component.

### Cost Per 10k Miles Metric
**Impact**: Medium | **Effort**: Low

Add a normalized cost metric for easier comparison:
- Calculate: `total_cost / (remaining_miles / 10000)`
- Display alongside annual cost
- Useful for comparing cars with different mileage

**Why**: Different remaining mileages make direct comparison difficult.

**Implementation**: Add calculation to `compute_car_data()` and display in `CarCostSummary`.

## Medium Priority - User Experience

### Input Validation & Error Handling
**Impact**: High | **Effort**: Medium

Improve form validation and error messages:
- Prevent negative values
- Validate MPG > 0, annual mileage > 0
- Show inline validation errors
- Add input formatting (currency, thousands separators)
- Disable calculations with invalid data

**Why**: Better UX and prevents calculation errors.

### Delete Confirmation Dialog
**Impact**: Medium | **Effort**: Low

Add confirmation before deleting cars:
- Modal dialog with car name
- "Are you sure?" message
- Cancel/Confirm buttons
- Prevent accidental deletions

**Why**: Deleting is currently instant with no undo.

### Mobile Responsive Design
**Impact**: High | **Effort**: Medium

Improve mobile layout:
- Stack form fields vertically on small screens
- Make tables scrollable horizontally
- Larger touch targets for buttons
- Collapsible sections to save space
- Test on various device sizes

**Why**: Many users will access on mobile devices.

### Tooltips & Help Text
**Impact**: Medium | **Effort**: Low

Add explanatory tooltips for complex fields:
- "What is opportunity cost?" tooltip
- Examples for each field (e.g., "If you drive 12,000 miles/year...")
- Info icons next to field labels
- Link to help documentation

**Why**: Users may not understand all financial concepts.

## Medium Priority - Data Management

### Maintenance Data Editing
**Impact**: Medium | **Effort**: Medium

Allow users to edit maintenance cost tables:
- Add/remove/edit data points
- Create new make/model entries
- Duplicate existing entries to modify
- Validate data (increasing costs, no negative values)

**Why**: Currently read-only, limiting customization.

### JSON Import/Export
**Impact**: Medium | **Effort**: Low

Allow users to backup and share data:
- Export all data (settings, cars, maintenance) as JSON
- Import JSON to restore or share configurations
- Export individual maintenance tables
- Community sharing of maintenance data

**Why**: Enables data portability and community contributions.

**Note**: This could help with CORS limitation - users could share scraped maintenance data as JSON files.

### More Sample Vehicles
**Impact**: Low | **Effort**: Medium

Expand the built-in maintenance database:
- Add 10-20 popular makes/models
- Include variety: sedans, SUVs, trucks, luxury, economy
- Source data from CarEdge, RepairPal, AAA
- Document data sources and dates

**Why**: More useful out-of-the-box experience.

## Low Priority - Polish

### Tagging System
**Impact**: Medium | **Effort**: Medium

Add tags to organize cars:
- Tag input with autocomplete
- Filter cars by tags
- Tag-based statistics (avg cost per tag)
- Color-coded tags
- Common tags: "finalist", "backup", "family car", etc.

**Why**: Helps organize large lists of cars.

### Search & Filter
**Impact**: Medium | **Effort**: Low

Add search and filtering to car list:
- Search by make/model/year
- Filter by price range, TCO range, year range
- Quick filters: "under $30k TCO", "less than 5 years remaining"
- Combine filters

**Why**: Useful for large car lists.

### Sorting Options
**Impact**: Low | **Effort**: Low

Sort car list by different criteria:
- By TCO (ascending/descending)
- By annual cost
- By purchase price
- By year
- By make/model
- Manual drag-and-drop ordering

**Why**: Different sorting reveals different insights.

### Dark Mode
**Impact**: Low | **Effort**: Medium

Add dark theme option:
- Toggle in settings
- Dark color palette
- Persist preference
- Match system preference

**Why**: Better for low-light use, user preference.

### Keyboard Shortcuts
**Impact**: Low | **Effort**: Low

Add keyboard navigation:
- `n` - New car
- `e` - Expand/collapse selected car
- Arrow keys - Navigate between cars
- `Delete` - Delete selected car (with confirmation)
- `?` - Show shortcuts help

**Why**: Power users appreciate keyboard navigation.

## Future Enhancements

### Loan Calculator Integration
**Impact**: High | **Effort**: High

Factor in auto loan costs:
- Loan amount, interest rate, term
- Calculate monthly payment
- Add interest to TCO
- Compare financing vs. cash purchase
- Show total interest paid

**Why**: Most cars are financed, making this calculation incomplete.

### Depreciation Tracking
**Impact**: Medium | **Effort**: High

Estimate vehicle value over time:
- Use industry depreciation curves
- Factor in make/model/year
- Show estimated resale value
- Net TCO (cost - residual value)

**Why**: More accurate ownership cost including resale.

### Electric Vehicle Support
**Impact**: Medium | **Effort**: Medium

Add EV-specific calculations:
- Electricity cost instead of gas
- kWh/mile instead of MPG
- Battery replacement costs
- Federal/state tax credits

**Why**: Growing EV market has different cost structure.

### Regional Cost Adjustments
**Impact**: Medium | **Effort**: High

Adjust costs by location:
- Gas prices by state/region
- Insurance rates by ZIP code
- Maintenance costs by area
- Registration/tax by location

**Why**: Significant regional variations in costs.

### Historical Cost Tracking
**Impact**: Low | **Effort**: High

Track actual costs vs. estimates:
- Log maintenance events with costs
- Track actual gas consumption
- Compare estimated vs. actual
- Adjust estimates based on actuals

**Why**: Improve accuracy over time with real data.

### Sharing & Collaboration
**Impact**: Low | **Effort**: High

Generate shareable links:
- Share car comparison with others
- View-only links (no editing)
- Embed comparison in websites
- Requires backend service

**Why**: Help others make decisions, show work to spouse/partner.

**Note**: Conflicts with frontend-only architecture, would require backend.

## Won't Do (Technical Limitations)

### Automated Data Fetching from CarEdge
**Reason**: CORS restrictions prevent browser-only apps from scraping external sites.
**Alternative**: JSON import/export for community data sharing.

### Real-time API Integration
**Reason**: No backend to proxy API requests, CORS blocks direct browser calls.
**Alternative**: Manual data entry with good UX.

### Multi-User Sync
**Reason**: Frontend-only architecture has no server for sync.
**Alternative**: JSON export/import for manual sharing.

## Implementation Notes

### Quick Wins (Low Effort, High Impact)
1. Cost per 10k miles metric
2. Delete confirmation dialog
3. Tooltips for complex fields
4. Input validation improvements

### Must-Haves for v1.0
1. Multi-car comparison view
2. Visual cost charts
3. Mobile responsive design
4. Input validation

### Nice-to-Haves
1. Tagging system
2. Dark mode
3. Keyboard shortcuts
4. More sample vehicles
