# CarCalc Implementation Roadmap

Last Updated: 2026-01-27

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

**Completed Files**:
- `src/lib.rs` - SharedSettingsForm component (lines 425-504)

**Outstanding Items**:
- [x] Add tooltips with detailed explanations for each field
- [ ] Add default value reset button
- [ ] Create percentage and currency formatters (currently using basic number inputs)

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
- [ ] Build search and filter functionality
- [ ] Add sorting options (price, year, make, etc.)
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
- [ ] Add manual data editing capability (currently read-only viewer)
- [ ] Add more sample vehicles to the database
- [ ] Add ability to import/export maintenance data as JSON

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

**Outstanding Items**:
- [ ] Build comparison table for multiple cars side-by-side
- [ ] Add charts/graphs (bar charts for cost categories)
- [ ] Implement different view modes (total, annual, per-mile)
- [ ] Create export functionality (CSV, print view)
- [ ] Add "what-if" scenario calculator (adjust settings temporarily)
- [ ] Add cost per 10k miles metric

**Files to Create**:
- `src/components/costs/comparison.rs` - Multi-car comparison view
- `src/components/costs/charts.rs` - Visual charts
- `src/utils/export.rs` - Export to CSV/JSON

## Phase 8: Tagging & Filtering System
**Status**: NOT STARTED

**Goal**: Enable organization and comparison of vehicle subsets

**Tasks**:
1. Add tags field to Car model (already in model but not in UI)
2. Create tag management UI
3. Implement tag filtering in car list
4. Add multi-select filtering
5. Create tag-based comparison views
6. Build tag statistics (avg cost per tag, etc.)
7. Add tag color customization

**Files to Create**:
- `src/components/tags/mod.rs`
- `src/components/tags/tag_manager.rs`
- `src/components/tags/tag_filter.rs`

## Phase 9: Polish & User Experience
**Status**: NOT STARTED

**Goal**: Refine the interface and add quality-of-life features

**Tasks**:
1. Add loading states and skeleton screens
2. Implement error boundaries and user-friendly error messages
3. Create onboarding/tutorial for new users
4. Add keyboard shortcuts
5. Implement dark mode
6. Add responsive design improvements for mobile
7. Seed sample cars on first load
8. Add data import/export (JSON backup)
9. Add input validation and helpful error messages
10. Improve form UX (better number inputs, auto-formatting)

**Files to Create**:
- `src/components/ui/loading.rs`
- `src/components/ui/error_boundary.rs`
- `src/components/onboarding.rs`
- `src/utils/keyboard.rs`
- `src/utils/theme.rs`
- `src/utils/import_export.rs`

## Phase 10: Testing & Documentation
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
- [ ] Add end-to-end tests with wasm-bindgen-test
- [ ] Write more comprehensive inline documentation
- [ ] Create user guide/help documentation
- [ ] Add developer documentation
- [ ] Performance profiling and optimization
- [ ] Test edge cases (empty states, invalid inputs, extreme values)

**Files to Create**:
- `tests/calculations_test.rs` - Additional calculation tests
- `tests/integration_test.rs` - Storage integration tests
- `tests/ui_test.rs` - Component interaction tests
- `docs/USER_GUIDE.md` - End-user documentation
- `docs/DEVELOPMENT.md` - Developer setup and contribution guide

## Summary

**Overall Progress**: ~55% Complete (5.5 of 10 phases substantially complete)

**Completed Phases**:
1. Foundation & Infrastructure - 100%
2. Local Storage & Data Layer - 100%
3. Lifestyle Settings UI - 90%
4. Car Management UI - 80%
5. Cost Calculation Engine - 100%
6. Maintenance Cost Integration - 100% (manual entry only, auto-fetch blocked by CORS)

**In Progress**:
7. Cost Display & Visualization - 40% (needs comparison and charts)

**Not Started**:
8. Tagging & Filtering System - 0%
9. Polish & User Experience - 0%
10. Testing & Documentation - 30%

## Recommended Next Steps

### Option A: Complete Core Functionality (Recommended)
Focus on making the current features production-ready:
1. Add comparison view for multiple cars
2. Add visual cost breakdown charts
3. Improve input validation and error handling
4. Add basic responsive design fixes
5. Create simple user guide

### Option B: Maintenance Cost Management (Updated)
Improve manual maintenance data management:
1. Add ability to edit existing maintenance data
2. Create JSON import/export for maintenance tables
3. Build more comprehensive sample data library
4. Add UI to create new make/model entries
5. Add data validation and error handling

### Option C: Enhanced User Experience
Make the app more polished:
1. Add tagging system for car organization
2. Implement comparison table
3. Add data export functionality
4. Add keyboard shortcuts
5. Implement dark mode

### Option D: Testing & Reliability
Ensure production readiness:
1. Add comprehensive unit tests
2. Create integration tests
3. Add error boundaries
4. Improve error messages
5. Performance optimization
