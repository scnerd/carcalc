# CarCalc Implementation Roadmap

Last Updated: 2026-01-27

## ⚠️ Quick Issues to Address

These are issues encountered during testing that should be resolved soon:

- [ ] The "maintenance cost data" dropdown has no types to select from. Should auto-populate with makes+models from entered cars. If none exist, provide instructions on where to get data (preferably CarEdge). Maybe embed an iframe for easy copying.
- [ ] The "shared settings" panel cannot be collapsed, add collapse/expand functionality
- [ ] Styling is minimalistic. Consider adding more color (e.g., Material Design enhancements)
- [ ] Add favicon to the site
- [ ] Car header shows "annual cost" but needs more info, especially expected years remaining
- [ ] No support for electric/plugin hybrid vehicles - should add this
- [ ] VIN lookup would be useful (NHTSA probably provides this) to auto-populate make+model+year

---

## Phase 1: Foundation & Infrastructure
**Status**: COMPLETED

- [x] Set up Leptos project with Tailwind CSS
- [x] Configure Trunk for static site generation
- [x] Create project documentation (README.md, Claude.md)
- [x] Basic routing and navigation structure

## Phase 2: Local Storage & Data Layer
**Status**: COMPLETED

**Goal**: Implement persistent browser storage

- [x] Add localStorage persistence using leptos-use
- [x] Create data models (SharedSettings, Car, MaintenanceCostDatabase)
- [x] Implement serialization/deserialization with serde
- [x] Create sample maintenance data (Toyota Prius, Ford F-150)
- [x] Add error handling for storage operations

**Completed Files**:
- `src/lib.rs` - All data structures, models, and localStorage integration

**Note**: Original plan called for Turso/IndexedDB, but localStorage with leptos-use proved simpler and sufficient for the use case.

## Phase 3: Lifestyle Settings UI
**Status**: COMPLETED

**Goal**: Create the global settings interface

- [x] Create SharedSettings component
- [x] Build form inputs with validation
- [x] Implement number input formatters
- [x] Add save/load functionality from localStorage
- [x] Add help text explaining each field
- [x] Add tooltips with detailed explanations for each field

**Completed Files**:
- `src/lib.rs` - SharedSettingsForm component (lines 425-504)

**Outstanding Items**:
- [ ] Add default value reset button
- [ ] Create percentage and currency formatters (currently using basic number inputs)
- [ ] Add collapse/expand functionality to the settings panel

## Phase 4: Car Management UI
**Status**: COMPLETED

**Goal**: Build the car entry and management interface

- [x] Create CarList component (displays all cars)
- [x] Build CarForm component (add/edit cars)
- [x] Implement car detail view with expand/collapse
- [x] Add delete functionality
- [x] Add car count tracking

**Completed Files**:
- `src/lib.rs` - CarList component (lines 1022-1120)
- `src/lib.rs` - CarForm component (lines 591-740)
- `src/lib.rs` - CarCard component (lines 743-852)

**Outstanding Items**:
- [ ] Add delete confirmation dialog (currently deletes immediately)
- [ ] Create tag input and management system
- [ ] Build search and filter functionality (by make/model/year, price/TCO range)
- [ ] Add sorting options (price, year, make, TCO, annual cost, etc.)
- [ ] Add bulk operations (delete multiple, compare selected)

## Phase 5: Cost Calculation Engine
**Status**: COMPLETED

**Goal**: Implement the core TCO calculation logic

- [x] Create calculation function with unit tests
- [x] Implement gas cost calculation
- [x] Implement insurance cost calculation
- [x] Implement opportunity cost calculation
- [x] Implement maintenance cost calculation (50/50 split mileage/time)
- [x] Add calculation validation and error handling
- [x] Write comprehensive unit tests

**Completed Files**:
- `src/lib.rs` - compute_car_data function (lines 314-392)
- `src/lib.rs` - Test suite (lines 1142-1277)

**Outstanding Items**:
- [ ] Add calculation result caching for performance
- [ ] Create calculation history tracking
- [ ] Add more edge case tests (division by zero, negative values)
- [ ] Add cost per 10k miles metric (total_cost / (remaining_miles / 10000))
- [ ] Support for electric/plugin hybrid vehicles (electricity cost, kWh/mile, battery replacement)

## Phase 6: Maintenance Cost Integration
**Status**: COMPLETED (Manual Entry Only)

**Goal**: Store and manage maintenance cost data

- [x] Create maintenance cost data structure with interpolation
- [x] Implement by-mileage and by-time cost tracking
- [x] Build caching layer (localStorage)
- [x] Create manual data entry UI (MaintenanceDataEditor)
- [x] Add sample data for common vehicles

**Completed Files**:
- `src/lib.rs` - MaintenanceCostData and MaintenanceCostDatabase (lines 30-262)
- `src/lib.rs` - MaintenanceDataEditor component (lines 855-1019)

**Outstanding Items**:
- [ ] Allow users to edit existing maintenance data (add/remove/edit data points)
- [ ] Create new make/model entries in UI
- [ ] Add ability to import/export maintenance data as JSON
- [ ] Add data validation (increasing costs, no negative values)
- [ ] Expand built-in maintenance database with 10-20 popular makes/models (sedans, SUVs, trucks, luxury, economy)
- [ ] Auto-populate maintenance cost dropdown with makes+models from user's entered cars
- [ ] Provide help/instructions for finding maintenance data

**Won't Do (Browser-Only Limitation)**:
- ~~Automated data fetching from CarEdge.com~~ - CORS restrictions prevent browser-only apps from scraping external sites. Users must manually enter or import maintenance data.

**Note**: A future backend service could fetch and aggregate maintenance data, but that conflicts with the privacy-first, frontend-only architecture. Manual entry ensures data privacy and offline capability.

## Phase 7: Cost Display & Visualization
**Status**: COMPLETED (Basic)

**Goal**: Present TCO calculations in clear, actionable formats

- [x] Create CostBreakdown component (CarCostSummary)
- [x] Display total and annual costs prominently
- [x] Show individual cost categories

**Completed Files**:
- `src/lib.rs` - CarCostSummary component (lines 507-588)

**Outstanding Items - High Priority**:
- [ ] Build comparison table for multiple cars side-by-side (sortable columns, highlight best/worst values)
- [ ] Add charts/graphs (stacked bar chart for TCO breakdown, pie chart for cost percentages)
- [ ] Add cost per 10k miles metric for easier comparison
- [ ] Create export functionality (CSV, print view)

**Outstanding Items - Medium Priority**:
- [ ] Implement different view modes (total, annual, per-mile)
- [ ] Add "what-if" scenario calculator (adjust settings temporarily)
- [ ] Add interactive tooltips with exact values on charts

**Files to Create**:
- `src/components/costs/comparison.rs` - Multi-car comparison view
- `src/components/costs/charts.rs` - Visual charts

## Phase 8: Input Validation & Error Handling
**Status**: NOT STARTED

**Goal**: Improve form validation and user feedback

- [ ] Prevent negative values in all inputs
- [ ] Validate MPG > 0, annual mileage > 0, maintenance costs valid
- [ ] Show inline validation errors
- [ ] Add input formatting (currency, thousands separators)
- [ ] Disable calculations with invalid data
- [ ] Add helpful error messages for edge cases

**Files to Create**:
- `src/utils/validation.rs` - Validation logic

## Phase 9: Tagging & Filtering System
**Status**: NOT STARTED

**Goal**: Enable organization and comparison of vehicle subsets

**Tasks**:
1. Add tag management UI (input with autocomplete)
2. Implement tag filtering in car list
3. Add multi-select filtering
4. Create tag-based comparison views
5. Build tag statistics (avg cost per tag, etc.)
6. Add tag color customization

**Files to Create**:
- `src/components/tags/mod.rs`
- `src/components/tags/tag_manager.rs`
- `src/components/tags/tag_filter.rs`

## Phase 10: Search, Filter & Sort
**Status**: NOT STARTED

**Goal**: Make car lists easier to navigate

**Tasks**:
1. Add search by make/model/year/notes
2. Add filter by price range, TCO range, year range
3. Create quick filters: "under $30k TCO", "less than 5 years remaining"
4. Implement sorting (by TCO, annual cost, purchase price, year, make/model)
5. Add manual drag-and-drop ordering
6. Combine multiple filters

**Files to Create**:
- `src/components/search/mod.rs`
- `src/components/filter/mod.rs`

## Phase 11: Polish & User Experience
**Status**: NOT STARTED

**Goal**: Refine the interface and add quality-of-life features

**Tasks**:
1. Add loading states and skeleton screens
2. Implement error boundaries and user-friendly error messages
3. Create onboarding/tutorial for new users
4. Add keyboard shortcuts (n=new car, e=expand, arrow keys=navigate, ?=help)
5. Implement dark mode with toggle and system preference
6. Add responsive design improvements for mobile (stack fields, scrollable tables, larger touch targets)
7. Seed sample cars on first load
8. Add data import/export (JSON backup)
9. Add favicon
10. Improve form UX (better number inputs, auto-formatting)

**Files to Create**:
- `src/components/ui/loading.rs`
- `src/components/ui/error_boundary.rs`
- `src/components/onboarding.rs`
- `src/utils/keyboard.rs`
- `src/utils/theme.rs`
- `src/utils/import_export.rs`

## Phase 12: Testing & Documentation
**Status**: PARTIALLY COMPLETED

**Goal**: Ensure reliability and maintainability

- [x] Write unit tests for calculation logic
- [x] Test maintenance cost interpolation
- [x] Test 50/50 mileage/time split
- [x] Test sample data integrity

**Completed Files**:
- `src/lib.rs` - Test module (lines 1142-1277)

**Outstanding Items**:
- [ ] Add integration tests for localStorage operations
- [ ] Create component tests for UI interactions
- [ ] Add end-to-end tests with Playwright
- [ ] Write more comprehensive inline documentation
- [ ] Create user guide/help documentation
- [ ] Add developer documentation
- [ ] Performance profiling and optimization
- [ ] Test edge cases (empty states, invalid inputs, extreme values)

**Files to Create**:
- `tests/calculations_test.rs` - Additional calculation tests
- `tests/integration_test.rs` - Storage integration tests
- `tests/e2e/` - Playwright e2e tests
- `docs/USER_GUIDE.md` - End-user documentation
- `docs/DEVELOPMENT.md` - Developer setup and contribution guide

---

## Future Enhancements (Post-MVP)

### High Impact Features

**Multi-Car Comparison View** | Effort: Medium
- Side-by-side comparison table with all key metrics
- Sortable columns (TCO, annual cost, years remaining, etc.)
- Highlight best/worst values in each category
- Sticky header for scrolling
- Export comparison as CSV or print view
- *Why*: Killer feature that makes the app truly valuable. Without it, users must mentally compare cars.

**Visual Cost Breakdown Charts** | Effort: Medium
- Stacked bar chart showing TCO breakdown by category
- Pie chart showing percentage of each cost type
- Side-by-side bars comparing multiple cars
- Interactive tooltips with exact values
- *Why*: Visual representation makes cost differences immediately obvious.

**Mobile Responsive Design** | Effort: Medium
- Stack form fields vertically on small screens
- Make tables scrollable horizontally
- Larger touch targets for buttons
- Collapsible sections to save space
- Test on various device sizes
- *Why*: Many users will access on mobile devices.

**Delete Confirmation Dialog** | Effort: Low
- Modal dialog with car name
- "Are you sure?" message
- Cancel/Confirm buttons
- Prevent accidental deletions
- *Why*: Deleting is currently instant with no undo.

### Medium Priority Features

**Loan Calculator Integration** | Effort: High
- Factor in auto loan costs (amount, interest rate, term)
- Calculate monthly payment
- Add interest to TCO
- Compare financing vs. cash purchase
- Show total interest paid
- *Why*: Most cars are financed, making this calculation incomplete.

**Depreciation Tracking** | Effort: High
- Use industry depreciation curves
- Factor in make/model/year
- Show estimated resale value
- Calculate net TCO (cost - residual value)
- *Why*: More accurate ownership cost including resale.

**Electric Vehicle Support** | Effort: Medium
- Electricity cost instead of gas
- kWh/mile instead of MPG
- Battery replacement costs
- Federal/state tax credits
- *Why*: Growing EV market has different cost structure.

**Regional Cost Adjustments** | Effort: High
- Gas prices by state/region
- Insurance rates by ZIP code
- Maintenance costs by area
- Registration/tax by location
- *Why*: Significant regional variations in costs.

### Lower Priority Enhancements

**Dark Mode** | Effort: Medium
- Toggle in settings
- Dark color palette
- Persist preference
- Match system preference
- *Why*: Better for low-light use, user preference.

**Keyboard Shortcuts** | Effort: Low
- `n` - New car
- `e` - Expand/collapse selected car
- Arrow keys - Navigate between cars
- `Delete` - Delete selected car (with confirmation)
- `?` - Show shortcuts help
- *Why*: Power users appreciate keyboard navigation.

**Historical Cost Tracking** | Effort: High
- Log maintenance events with costs
- Track actual gas consumption
- Compare estimated vs. actual
- Adjust estimates based on actuals
- *Why*: Improve accuracy over time with real data.

**Sharing & Collaboration** | Effort: High
- Generate shareable links
- Share car comparison with others
- View-only links (no editing)
- Embed comparison in websites
- *Note*: Conflicts with frontend-only architecture, would require backend.

## Won't Do (Technical Limitations)

- **Automated Data Fetching from CarEdge**: CORS restrictions prevent browser-only apps from scraping. Alternative: JSON import/export for community data sharing.
- **Real-time API Integration**: No backend to proxy, CORS blocks direct browser calls. Alternative: Manual data entry with good UX.
- **Multi-User Sync**: Frontend-only has no server. Alternative: JSON export/import for manual sharing.

---

## Summary

**Overall Progress**: ~55% Complete (5.5 of 12 phases substantially complete)

**Completed Phases**:
1. Foundation & Infrastructure - 100%
2. Local Storage & Data Layer - 100%
3. Lifestyle Settings UI - 90%
4. Car Management UI - 80%
5. Cost Calculation Engine - 100%
6. Maintenance Cost Integration - 100% (manual entry only)

**In Progress**:
7. Cost Display & Visualization - 40% (needs comparison and charts)

**Not Started**:
8. Input Validation & Error Handling - 0%
9. Tagging & Filtering System - 0%
10. Search, Filter & Sort - 0%
11. Polish & User Experience - 0%
12. Testing & Documentation - 30%

---

## Recommended Next Steps

### Option A: Complete Core Functionality (Recommended)
Focus on making the current features production-ready:
1. Add comparison view for multiple cars
2. Add visual cost breakdown charts
3. Improve input validation and error handling
4. Add basic responsive design fixes
5. Create simple user guide

### Option B: Quick Wins (Low Effort, High Impact)
Easy items that improve user experience quickly:
1. Delete confirmation dialog
2. Cost per 10k miles metric
3. Expand/collapse settings panel
4. Add delete confirmation
5. Auto-populate maintenance cost dropdown with user's cars

### Option C: Maintenance Cost Management (Updated)
Improve manual maintenance data management:
1. Add ability to edit existing maintenance data
2. Create JSON import/export for maintenance tables
3. Auto-populate dropdown with user's makes/models
4. Build more comprehensive sample data library
5. Add UI to create new make/model entries

### Option D: Enhanced User Experience
Make the app more polished:
1. Add tagging system for car organization
2. Implement comparison table
3. Add data export functionality
4. Add keyboard shortcuts
5. Implement dark mode

### Option E: Testing & Reliability
Ensure production readiness:
1. Add comprehensive unit tests
2. Create integration tests
3. Add error boundaries
4. Improve error messages
5. Performance optimization
