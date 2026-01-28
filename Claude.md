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

### Development Workflow with Taskfile

This project uses [Task](https://taskfile.dev/) for cross-platform task automation. All commands work on both Windows CMD and Bash.

#### Quick Reference for Claude Code

When working on this project, use these Task commands optimized for Claude Code workflows:

**Starting Development**
```bash
task dev          # Start dev server with hot reload at http://127.0.0.1:8080
```

**Code Quality Checks (Run Before Committing)**
```bash
task ci           # Complete CI pipeline: format check, clippy, and tests
```

**Individual Quality Checks**
```bash
task fmt          # Format code with rustfmt
task fmt:check    # Check formatting without modifying
task clippy       # Run clippy linter
task test         # Run all tests
task check        # Quick compilation check
```

**Building**
```bash
task build              # Development build
task build:release      # Optimized production build
```

**Testing**
```bash
task test              # Run Rust unit tests
task test:e2e          # Run Playwright e2e tests (builds first)
task test:e2e:ui       # Run Playwright tests in UI mode
```

**Maintenance**
```bash
task clean             # Clean build artifacts
task install           # Install Rust WASM target and Trunk
task install:playwright # Install Playwright and e2e test dependencies
```

#### Recommended Claude Code Workflow

1. **Before making changes**: Run `task check` to ensure the project compiles
2. **While developing**: Keep `task dev` running in background to see live updates
3. **After changes**: Run `task ci` to validate all checks pass
4. **Before committing**: Ensure `task ci` passes successfully

#### Why Use Task Commands

✅ **Prefer Task commands** (e.g., `task test`, `task ci`) because:
- Cross-platform compatibility guaranteed
- Consistent interface across all operations
- Combined operations like `task ci` run multiple checks
- Easier for users to reproduce

❌ **Avoid direct cargo/trunk commands** unless needed for:
- Advanced cargo features not exposed in Taskfile
- Debugging specific compilation issues
- Custom flags not covered by Task commands

### Code Style
- Use `task fmt` for consistent formatting
- Run `task clippy` and address all warnings before committing
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

### End-to-End Testing with Playwright

**Setup Requirements**
When setting up Playwright for the first time or in a new environment:
1. Run `task install:playwright` to install all dependencies
2. This installs: npm packages, wasm-bindgen-cli (matching version 0.2.108), and Chromium browser

**Important Notes**
- Playwright version pinned to 1.56.1 to match pre-installed binaries in some environments
- wasm-bindgen-cli must match the version in Cargo.toml (0.2.108)
- Tests should be placed in `tests/` directory with `.spec.js` or `.spec.ts` extension
- Dev server must be running at http://localhost:8080 for tests to work
- Use `task test:e2e` to automatically build and run tests
- Use `task test:e2e:ui` for interactive test debugging

**Test Organization**
```
tests/
├── e2e/
│   ├── home.spec.js           # Homepage and navigation tests
│   ├── lifestyle.spec.js      # Lifestyle settings tests
│   ├── cars.spec.js           # Car CRUD operations
│   └── calculations.spec.js   # TCO calculation verification
└── playwright.config.js       # Playwright configuration
```

**Common Test Patterns**
```javascript
// Wait for WASM to initialize
await page.waitForLoadState('networkidle');
await page.waitForTimeout(1000); // Additional buffer for WASM

// Test navigation
await page.goto('http://localhost:8080');
await expect(page).toHaveTitle(/CarCalc/);

// Interact with forms
await page.fill('input[name="annual_mileage"]', '12000');
await page.click('button[type="submit"]');

// Verify localStorage persistence
const storage = await page.evaluate(() => localStorage.getItem('key'));
expect(JSON.parse(storage)).toMatchObject({...});
```

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
Project Status: ~55% Complete (Phases 1-5 substantially done)
Next: See TODO.md for recommended next steps
Perfect! I've succes
