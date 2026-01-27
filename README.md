# CarCalc - Total Cost of Ownership Calculator

A frontend-only web application built with Rust and Leptos that helps you calculate the true total cost of owning any car.

## Overview

CarCalc provides a comprehensive analysis of vehicle ownership costs, going beyond the sticker price to include maintenance, insurance, fuel, and opportunity costs. All data is stored locally in your browser using IndexedDB (via Turso), ensuring privacy and offline functionality.

## Features

### Current Features
- Clean, responsive UI built with Leptos and Tailwind CSS
- Static site generation for fast, reliable hosting
- Local browser storage (no server required)

### Planned Features

#### Lifestyle Settings
- **Opportunity Cost Rate**: Default 8%, represents the potential investment return on capital tied up in vehicle purchase
- **Annual Mileage**: Average miles driven per year for calculating usage-based costs
- **Lifetime Miles**: Default total miles expected to drive a car (can be overridden per vehicle)
- **Average Gas Price**: Expected cost per gallon for fuel calculations

#### Vehicle Management
Each car entry tracks:
- Make, Model, Trim/Features (optional)
- Model Year
- Purchase Price
- Current Mileage
- MPG (Miles Per Gallon)
- Insurance Cost (6-month premium)
- Metadata (VIN, listing URL, notes, etc.)

#### Cost Analysis
The calculator provides comprehensive cost breakdowns:

1. **Remaining Miles**: Total lifetime miles minus current mileage
2. **Years Remaining**: Remaining miles divided by annual mileage
3. **Gas Cost**: (Remaining miles / MPG) × Cost per gallon
4. **Insurance Cost**: (6-month premium × 2) × Years remaining
5. **Opportunity Cost**: Purchase price × Opportunity rate × Years remaining
6. **Maintenance Cost**: Calculated from per-thousand-mile maintenance tables (sourced from CarEdge.com)

Results are presented in multiple formats:
- Total cost
- Annual cost (total / years remaining)
- Cost per 10,000 miles
- Individual cost breakdowns by category

#### Organization & Comparison
- **Tagging System**: Tag vehicles (e.g., "minivan", "commuter", "dream car") for easy comparison
- **Filtering**: View and compare specific categories of vehicles
- **Side-by-side Analysis**: Compare multiple vehicles to find the best value

#### Data Sources
- **Maintenance Costs**: Automatically pulled from CarEdge.com maintenance cost tables
- Costs are converted from annual to per-thousand-mile rates
- Costs adjust based on vehicle age/mileage (older cars cost more to maintain)

## Technology Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) (Rust)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)
- **Build Tool**: [Trunk](https://trunkrs.dev/)
- **Database**: [Turso](https://turso.tech/) (local browser storage)
- **Language**: Rust (compiles to WebAssembly)

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Trunk](https://trunkrs.dev/) - Install with: `cargo install trunk`
- [Tailwind CSS CLI](https://tailwindcss.com/docs/installation) - Install with npm: `npm install -D tailwindcss`
- [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/game-of-life/setup.html) - Install with: `rustup target add wasm32-unknown-unknown`

## Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd carcalc
   ```

2. **Install dependencies**
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   npm install -D tailwindcss
   ```

3. **Run development server**
   ```bash
   trunk serve
   ```
   The app will be available at `http://127.0.0.1:8080`

4. **Build for production**
   ```bash
   trunk build --release
   ```
   Static files will be in the `dist/` directory

## Project Structure

```
carcalc/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Main app component and routing
│   └── ...              # Feature modules (to be added)
├── index.html           # HTML template
├── input.css            # Tailwind CSS input
├── tailwind.config.js   # Tailwind configuration
├── Trunk.toml           # Trunk build configuration
├── Cargo.toml           # Rust dependencies
├── README.md            # This file
└── Claude.md            # AI context and implementation plan
```

## Development

### Running Tests
```bash
cargo test
```

### Linting
```bash
cargo clippy
```

### Formatting
```bash
cargo fmt
```

## Deployment

The built application is fully static and can be deployed to any static hosting service:
- GitHub Pages
- Netlify
- Vercel
- Cloudflare Pages
- AWS S3 + CloudFront
- Any web server

Simply build with `trunk build --release` and upload the `dist/` directory.

## Contributing

This is a personal project, but suggestions and bug reports are welcome. Please open an issue to discuss proposed changes.

## License

See LICENSE file for details.

## Roadmap

See `Claude.md` for the detailed implementation plan and development phases.

## Privacy

All data is stored locally in your browser. No information is sent to external servers. The app can function completely offline after the initial load.
