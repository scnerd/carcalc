# Claude.md - CarCalc Project Context

## Project Overview

CarCalc is a frontend-only web application for calculating the total cost of ownership (TCO) for vehicles. It helps users make informed decisions by showing the complete financial picture of car ownership, including often-overlooked costs like opportunity cost and age-based maintenance increases.

### Technology Choices

- **Leptos (Rust)**: Type-safe, high-performance reactive framework that compiles to WebAssembly
- **Frontend-only**: No backend server required, all computation happens in the browser
- **Turso Database**: Embedded in browser via IndexedDB for local data persistence
- **Tailwind CSS**: Utility-first CSS framework for rapid UI development
- **Static Deployment**: Builds to static files for simple, reliable hosting

### Key Design Principles

1. **Privacy-First**: All data stays in the user's browser
2. **Offline-Capable**: Works without internet connection after initial load
3. **Type Safety**: Leverage Rust's type system to prevent bugs
4. **Performance**: WebAssembly provides near-native performance
5. **Simplicity**: Clean, intuitive interface focused on the core use case

## Core Functionality

### Data Model

#### Lifestyle Settings (Global)
```rust
struct LifestyleSettings {
    opportunity_cost_rate: f64,  // Default: 0.08 (8%)
    annual_mileage: u32,          // Miles driven per year
    lifetime_miles: u32,          // Default total miles for a car
    avg_gas_price: f64,           // Cost per gallon
}
```

#### Car Entry
```rust
struct Car {
    id: String,                   // UUID
    make: String,
    model: String,
    trim: Option<String>,
    year: u16,
    purchase_price: f64,
    current_mileage: u32,
    mpg: f64,
    insurance_premium_6mo: f64,
    vin: Option<String>,
    listing_url: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    lifetime_miles_override: Option<u32>,
    created_at: DateTime,
    updated_at: DateTime,
}
```

#### Maintenance Cost Table Entry
```rust
struct MaintenanceCost {
    make: String,
    model: String,
    year_range: (u16, u16),       // e.g., (2015, 2020)
    cost_per_thousand_miles: Vec<f64>, // Index = thousand mile marker
}
```

### Cost Calculation Logic

```
1. Remaining Miles = (lifetime_miles OR lifetime_miles_override) - current_mileage
2. Years Remaining = remaining_miles / annual_mileage
3. Gas Cost = (remaining_miles / mpg) × avg_gas_price
4. Insurance Cost = (insurance_premium_6mo × 2) × years_remaining
5. Opportunity Cost = purchase_price × opportunity_cost_rate × years_remaining
6. Maintenance Cost = Σ(cost for each 1000-mile increment from current to lifetime)
7. Total Cost = gas + insurance + opportunity + maintenance

Outputs:
- Total Cost
- Annual Cost = total / years_remaining
- Cost per 10k Miles = total / (remaining_miles / 10000)
- Individual category subtotals
```

### Features

1. **Lifestyle Configuration**: Single settings page for global assumptions
2. **Car Management**: Add, edit, delete, and organize cars
3. **Automatic Data Enrichment**: Fetch maintenance costs from CarEdge.com
4. **Tagging System**: Categorize cars for easy filtering and comparison
5. **Cost Visualization**: Clear presentation of TCO breakdowns
6. **Comparison View**: Side-by-side analysis of multiple vehicles

## Implementation Plan

### Phase 1: Foundation & Infrastructure ✅
**Status**: COMPLETED

- [x] Set up Leptos project with Tailwind CSS
- [x] Configure Trunk for static site generation
- [x] Create project documentation (README.md, Claude.md)
- [x] Basic routing and navigation structure

### Phase 2: Local Storage & Data Layer
**Goal**: Implement persistent browser storage with Turso/IndexedDB

Tasks:
1. Add Turso database dependencies and setup
2. Create database schema and migrations
3. Implement data models (LifestyleSettings, Car, MaintenanceCost)
4. Build CRUD operations for each model
5. Add serialization/deserialization helpers
6. Create database initialization and seed data
7. Implement error handling for storage operations

**Files to Create**:
- `src/db/mod.rs` - Database initialization and connection
- `src/db/schema.rs` - Table definitions
- `src/models/mod.rs` - Data models
- `src/models/lifestyle.rs` - Lifestyle settings model
- `src/models/car.rs` - Car model
- `src/models/maintenance.rs` - Maintenance cost model
- `src/db/operations.rs` - CRUD operations

### Phase 3: Lifestyle Settings UI
**Goal**: Create the global settings interface

Tasks:
1. Create LifestyleSettings component
2. Build form inputs with validation
3. Implement percentage, currency, and mileage formatters
4. Add save/load functionality from database
5. Create help text and tooltips explaining each field
6. Add default value reset options

**Files to Create**:
- `src/components/lifestyle_settings.rs`
- `src/components/forms/mod.rs`
- `src/utils/formatters.rs`

### Phase 4: Car Management UI
**Goal**: Build the car entry and management interface

Tasks:
1. Create CarList component (displays all cars)
2. Build CarForm component (add/edit cars)
3. Implement car detail view
4. Add delete confirmation dialog
5. Create tag input and management
6. Build search and filter functionality
7. Add sorting options (price, year, make, etc.)

**Files to Create**:
- `src/components/cars/mod.rs`
- `src/components/cars/car_list.rs`
- `src/components/cars/car_form.rs`
- `src/components/cars/car_detail.rs`
- `src/components/cars/car_card.rs`
- `src/components/ui/dialog.rs`
- `src/components/ui/tag_input.rs`

### Phase 5: Cost Calculation Engine
**Goal**: Implement the core TCO calculation logic

Tasks:
1. Create calculation module with unit tests
2. Implement each cost component (gas, insurance, opportunity, maintenance)
3. Add calculation result caching for performance
4. Create calculation history tracking
5. Build error handling for edge cases (zero MPG, negative values, etc.)
6. Add calculation validation

**Files to Create**:
- `src/calculations/mod.rs`
- `src/calculations/tco.rs`
- `src/calculations/maintenance.rs`
- `src/calculations/tests.rs`

### Phase 6: Maintenance Cost Integration
**Goal**: Fetch and store maintenance cost data from CarEdge.com

Tasks:
1. Research CarEdge.com API or scraping approach
2. Implement data fetching mechanism
3. Create parsers for maintenance cost data
4. Convert annual costs to per-thousand-mile costs
5. Build caching layer for fetched data
6. Add manual override capability
7. Implement fallback for unavailable data

**Files to Create**:
- `src/services/caredge.rs`
- `src/services/http_client.rs`
- `src/parsers/maintenance_data.rs`

### Phase 7: Cost Display & Visualization
**Goal**: Present TCO calculations in clear, actionable formats

Tasks:
1. Create CostBreakdown component
2. Build comparison table for multiple cars
3. Add charts/graphs (bar charts for cost categories)
4. Implement different view modes (total, annual, per-mile)
5. Create export functionality (CSV, print view)
6. Add "what-if" scenario calculator

**Files to Create**:
- `src/components/costs/mod.rs`
- `src/components/costs/breakdown.rs`
- `src/components/costs/comparison.rs`
- `src/components/costs/charts.rs`
- `src/utils/export.rs`

### Phase 8: Tagging & Filtering System
**Goal**: Enable organization and comparison of vehicle subsets

Tasks:
1. Create tag management UI
2. Implement tag filtering in car list
3. Add multi-select filtering
4. Create tag-based comparison views
5. Build tag statistics (avg cost per tag, etc.)
6. Add tag color customization

**Files to Create**:
- `src/components/tags/mod.rs`
- `src/components/tags/tag_manager.rs`
- `src/components/tags/tag_filter.rs`

### Phase 9: Polish & User Experience
**Goal**: Refine the interface and add quality-of-life features

Tasks:
1. Add loading states and skeleton screens
2. Implement error boundaries and user-friendly error messages
3. Create onboarding/tutorial for new users
4. Add keyboard shortcuts
5. Implement dark mode
6. Add responsive design improvements for mobile
7. Create example cars with sample data
8. Add data import/export (JSON backup)

**Files to Create**:
- `src/components/ui/loading.rs`
- `src/components/ui/error_boundary.rs`
- `src/components/onboarding.rs`
- `src/utils/keyboard.rs`
- `src/utils/theme.rs`

### Phase 10: Testing & Documentation
**Goal**: Ensure reliability and maintainability

Tasks:
1. Write unit tests for all calculation logic
2. Add integration tests for database operations
3. Create component tests for UI
4. Add end-to-end tests with wasm-bindgen-test
5. Write inline documentation
6. Create user guide/help documentation
7. Add developer documentation
8. Performance profiling and optimization

**Files to Create**:
- `tests/calculations_test.rs`
- `tests/integration_test.rs`
- `tests/ui_test.rs`
- `docs/USER_GUIDE.md`
- `docs/DEVELOPMENT.md`

## Architecture Decisions

### Why Frontend-Only?
- Eliminates server costs and complexity
- Ensures user privacy (data never leaves their browser)
- Enables offline functionality
- Simplifies deployment (just static files)
- Reduces attack surface (no backend to compromise)

### Why Rust/Leptos?
- Type safety prevents entire classes of bugs
- Excellent performance (WebAssembly)
- Modern reactive framework similar to React
- Great developer experience with helpful compiler
- Single language for entire frontend

### Why Turso in Browser?
- SQLite-compatible (familiar SQL interface)
- Designed for edge/local deployments
- Full relational database features
- No network latency for queries
- Automatic persistence to IndexedDB

## Development Guidelines

### Code Style
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` and address all warnings
- Write descriptive variable and function names
- Add inline comments for complex logic
- Keep functions small and focused (single responsibility)

### Component Structure
```rust
// Standard Leptos component structure
#[component]
pub fn ComponentName(
    // Props with default values where appropriate
    #[prop(default = "default value")] prop_name: Type,
) -> impl IntoView {
    // Signals and state
    let (state, set_state) = create_signal(initial_value);

    // Effects and derived state

    // Event handlers

    // View
    view! {
        // JSX-like markup
    }
}
```

### Database Patterns
- Use transactions for multi-step operations
- Always handle database errors gracefully
- Provide fallback values for missing data
- Cache frequently accessed data in signals
- Debounce writes to avoid excessive storage operations

### Testing Strategy
- Unit test all calculation logic
- Test edge cases (zero, negative, very large numbers)
- Mock database operations in component tests
- Test user interactions (clicks, form submissions)
- Verify accessibility (keyboard navigation, screen readers)

## Future Enhancements

### Potential Features (Post-MVP)
1. **Loan Calculator**: Factor in interest on auto loans
2. **Depreciation Tracking**: Estimate vehicle value over time
3. **Fuel Type Support**: Electric, hybrid calculations
4. **Regional Cost Adjustments**: Gas prices, insurance rates by location
5. **Historical Tracking**: Track actual costs vs estimates
6. **Multi-Vehicle Households**: Compare fleet total costs
7. **Sharing**: Generate shareable links to car comparisons
8. **Mobile App**: Native mobile apps with Tauri
9. **API Integration**: Auto-populate data from VIN lookup services
10. **Advanced Analytics**: Trends, predictions, recommendations

### Scalability Considerations
- Current architecture supports thousands of cars per user
- IndexedDB limits: ~50MB typical, can request more
- Consider data pruning/archiving after many years
- Turso can sync to cloud for backup (future enhancement)

## Common Pitfalls to Avoid

1. **Floating Point Precision**: Use proper rounding for currency (2 decimal places)
2. **Division by Zero**: Check MPG, annual mileage before dividing
3. **Negative Values**: Validate all inputs are positive where required
4. **Missing Data**: Handle cases where maintenance costs aren't available
5. **Type Conversions**: Be careful with u32 ↔ f64 conversions
6. **Async Operations**: Remember database operations are async
7. **Memory Leaks**: Clean up event listeners and effects
8. **State Management**: Keep state minimal and derived values computed

## Questions to Resolve

1. **CarEdge Integration**: Do they have an API, or do we need to scrape?
2. **Maintenance Data Structure**: How granular? Every 1k miles? By age?
3. **Currency**: Support international currencies or USD only?
4. **Mileage Units**: Support kilometers as well as miles?
5. **Tax Calculations**: Include sales tax, registration fees?
6. **Insurance**: Allow multiple insurance quotes per car?

## Resources

- [Leptos Documentation](https://leptos.dev/)
- [Leptos Book](https://book.leptos.dev/)
- [Turso Docs](https://docs.turso.tech/)
- [Tailwind CSS Docs](https://tailwindcss.com/docs)
- [Trunk Guide](https://trunkrs.dev/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)

---

Last Updated: 2026-01-27
Project Status: Phase 1 Complete
Next Phase: Local Storage & Data Layer
