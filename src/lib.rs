use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Data Structures

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SharedSettings {
    pub opportunity_cost_rate: f64,
    pub annual_mileage: f64,
    pub lifetime_miles: f64,
    pub average_gas_price: f64,
}

impl Default for SharedSettings {
    fn default() -> Self {
        Self {
            opportunity_cost_rate: 8.0,
            annual_mileage: 12000.0,
            lifetime_miles: 200000.0,
            average_gas_price: 3.50,
        }
    }
}

/// Represents a single data point in a maintenance cost table
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceDataPoint {
    pub x: f64,  // mileage (in 10k miles) or years
    pub y: f64,  // cumulative cost in dollars
}

/// Maintenance cost data for a specific make+model
/// Contains two tables: one based on mileage, one based on time
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceCostData {
    pub make: String,
    pub model: String,
    /// Data points where x = 10k miles, y = cumulative cost over those miles
    pub by_mileage: Vec<MaintenanceDataPoint>,
    /// Data points where x = years, y = cumulative cost over those years
    pub by_time: Vec<MaintenanceDataPoint>,
}

impl MaintenanceCostData {
    fn new(make: String, model: String) -> Self {
        Self {
            make,
            model,
            by_mileage: Vec::new(),
            by_time: Vec::new(),
        }
    }

    /// Get a unique key for this make+model combination
    pub fn key(&self) -> String {
        format!("{}_{}", self.make.to_lowercase(), self.model.to_lowercase())
    }

    /// Create a key from make and model strings
    pub fn make_key(make: &str, model: &str) -> String {
        format!("{}_{}", make.to_lowercase(), model.to_lowercase())
    }

    /// Calculate maintenance cost for a given mileage range
    /// Uses linear interpolation between data points
    pub fn cost_for_mileage_range(&self, start_miles: f64, end_miles: f64) -> f64 {
        if self.by_mileage.is_empty() || end_miles <= start_miles {
            return 0.0;
        }

        let start_10k = start_miles / 10000.0;
        let end_10k = end_miles / 10000.0;

        let start_cost = self.interpolate_cost(&self.by_mileage, start_10k);
        let end_cost = self.interpolate_cost(&self.by_mileage, end_10k);

        (end_cost - start_cost).max(0.0)
    }

    /// Calculate maintenance cost for a given time range
    /// Uses linear interpolation between data points
    pub fn cost_for_time_range(&self, start_years: f64, end_years: f64) -> f64 {
        if self.by_time.is_empty() || end_years <= start_years {
            return 0.0;
        }

        let start_cost = self.interpolate_cost(&self.by_time, start_years);
        let end_cost = self.interpolate_cost(&self.by_time, end_years);

        (end_cost - start_cost).max(0.0)
    }

    /// Interpolate cost at a given x value from a series of data points
    fn interpolate_cost(&self, data: &[MaintenanceDataPoint], x: f64) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        // If before first point, extrapolate linearly from origin
        if x <= data[0].x {
            if data[0].x == 0.0 {
                return data[0].y;
            }
            return (data[0].y / data[0].x) * x;
        }

        // If after last point, extrapolate using last two points
        if x >= data[data.len() - 1].x {
            if data.len() == 1 {
                // Only one point, extrapolate from origin
                return (data[0].y / data[0].x) * x;
            }
            let p1 = &data[data.len() - 2];
            let p2 = &data[data.len() - 1];
            let slope = (p2.y - p1.y) / (p2.x - p1.x);
            return p2.y + slope * (x - p2.x);
        }

        // Find the two points to interpolate between
        for i in 0..data.len() - 1 {
            if x >= data[i].x && x <= data[i + 1].x {
                let p1 = &data[i];
                let p2 = &data[i + 1];

                if p2.x == p1.x {
                    return p1.y;
                }

                let ratio = (x - p1.x) / (p2.x - p1.x);
                return p1.y + ratio * (p2.y - p1.y);
            }
        }

        0.0
    }
}

/// Storage for all maintenance cost data, keyed by make_model
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct MaintenanceCostDatabase {
    pub data: HashMap<String, MaintenanceCostData>,
}

impl MaintenanceCostDatabase {
    /// Get maintenance data for a specific make+model
    pub fn get(&self, make: &str, model: &str) -> Option<&MaintenanceCostData> {
        let key = MaintenanceCostData::make_key(make, model);
        self.data.get(&key)
    }

    /// Set maintenance data for a specific make+model
    pub fn set(&mut self, data: MaintenanceCostData) {
        let key = data.key();
        self.data.insert(key, data);
    }

    /// Remove maintenance data for a specific make+model
    pub fn remove(&mut self, make: &str, model: &str) {
        let key = MaintenanceCostData::make_key(make, model);
        self.data.remove(&key);
    }

    /// Get all make+model combinations that have maintenance data
    pub fn get_all_keys(&self) -> Vec<(String, String)> {
        self.data.values()
            .map(|d| (d.make.clone(), d.model.clone()))
            .collect()
    }
}

/// Sample maintenance cost data based on typical costs for popular vehicles
/// This data represents cumulative maintenance costs over time and mileage
pub fn get_sample_maintenance_data() -> MaintenanceCostDatabase {
    let mut db = MaintenanceCostDatabase::default();

    // Toyota Prius - known for reliability and lower maintenance costs
    // Based on typical maintenance schedules: oil changes, tire rotations, brake service, etc.
    let mut prius = MaintenanceCostData::new("Toyota".to_string(), "Prius".to_string());

    // By mileage (x = 10k miles increments)
    // Cumulative costs include: oil changes, filters, tire rotations, brake pads, fluids, etc.
    prius.by_mileage = vec![
        MaintenanceDataPoint { x: 1.0, y: 350.0 },    // 10k miles
        MaintenanceDataPoint { x: 2.0, y: 700.0 },    // 20k miles
        MaintenanceDataPoint { x: 3.0, y: 1100.0 },   // 30k miles (major service)
        MaintenanceDataPoint { x: 4.0, y: 1450.0 },   // 40k miles
        MaintenanceDataPoint { x: 5.0, y: 1800.0 },   // 50k miles
        MaintenanceDataPoint { x: 6.0, y: 2300.0 },   // 60k miles (major service)
        MaintenanceDataPoint { x: 7.0, y: 2700.0 },   // 70k miles
        MaintenanceDataPoint { x: 8.0, y: 3100.0 },   // 80k miles
        MaintenanceDataPoint { x: 9.0, y: 3650.0 },   // 90k miles (major service)
        MaintenanceDataPoint { x: 10.0, y: 4100.0 },  // 100k miles
        MaintenanceDataPoint { x: 12.0, y: 5200.0 },  // 120k miles (major service)
        MaintenanceDataPoint { x: 15.0, y: 6800.0 },  // 150k miles
        MaintenanceDataPoint { x: 20.0, y: 9500.0 },  // 200k miles
    ];

    // By time (x = years)
    // Split 50/50 with mileage-based costs, assuming 12k miles/year
    prius.by_time = vec![
        MaintenanceDataPoint { x: 1.0, y: 420.0 },    // 1 year
        MaintenanceDataPoint { x: 2.0, y: 840.0 },    // 2 years
        MaintenanceDataPoint { x: 3.0, y: 1320.0 },   // 3 years
        MaintenanceDataPoint { x: 4.0, y: 1740.0 },   // 4 years
        MaintenanceDataPoint { x: 5.0, y: 2160.0 },   // 5 years
        MaintenanceDataPoint { x: 6.0, y: 2760.0 },   // 6 years
        MaintenanceDataPoint { x: 7.0, y: 3240.0 },   // 7 years
        MaintenanceDataPoint { x: 8.0, y: 3720.0 },   // 8 years
        MaintenanceDataPoint { x: 9.0, y: 4380.0 },   // 9 years
        MaintenanceDataPoint { x: 10.0, y: 4920.0 },  // 10 years
        MaintenanceDataPoint { x: 12.0, y: 6240.0 },  // 12 years
        MaintenanceDataPoint { x: 15.0, y: 8160.0 },  // 15 years
    ];

    db.set(prius);

    // Ford F-150 - popular truck with higher maintenance costs
    // Larger engine, more fluids, heavier wear on components
    let mut f150 = MaintenanceCostData::new("Ford".to_string(), "F-150".to_string());

    // By mileage (x = 10k miles increments)
    f150.by_mileage = vec![
        MaintenanceDataPoint { x: 1.0, y: 500.0 },    // 10k miles
        MaintenanceDataPoint { x: 2.0, y: 1000.0 },   // 20k miles
        MaintenanceDataPoint { x: 3.0, y: 1600.0 },   // 30k miles (major service)
        MaintenanceDataPoint { x: 4.0, y: 2150.0 },   // 40k miles
        MaintenanceDataPoint { x: 5.0, y: 2700.0 },   // 50k miles
        MaintenanceDataPoint { x: 6.0, y: 3400.0 },   // 60k miles (major service)
        MaintenanceDataPoint { x: 7.0, y: 4000.0 },   // 70k miles
        MaintenanceDataPoint { x: 8.0, y: 4600.0 },   // 80k miles
        MaintenanceDataPoint { x: 9.0, y: 5350.0 },   // 90k miles (major service)
        MaintenanceDataPoint { x: 10.0, y: 6000.0 },  // 100k miles
        MaintenanceDataPoint { x: 12.0, y: 7600.0 },  // 120k miles (major service)
        MaintenanceDataPoint { x: 15.0, y: 10000.0 }, // 150k miles
        MaintenanceDataPoint { x: 20.0, y: 14000.0 }, // 200k miles
    ];

    // By time (x = years)
    f150.by_time = vec![
        MaintenanceDataPoint { x: 1.0, y: 600.0 },    // 1 year
        MaintenanceDataPoint { x: 2.0, y: 1200.0 },   // 2 years
        MaintenanceDataPoint { x: 3.0, y: 1920.0 },   // 3 years
        MaintenanceDataPoint { x: 4.0, y: 2580.0 },   // 4 years
        MaintenanceDataPoint { x: 5.0, y: 3240.0 },   // 5 years
        MaintenanceDataPoint { x: 6.0, y: 4080.0 },   // 6 years
        MaintenanceDataPoint { x: 7.0, y: 4800.0 },   // 7 years
        MaintenanceDataPoint { x: 8.0, y: 5520.0 },   // 8 years
        MaintenanceDataPoint { x: 9.0, y: 6420.0 },   // 9 years
        MaintenanceDataPoint { x: 10.0, y: 7200.0 },  // 10 years
        MaintenanceDataPoint { x: 12.0, y: 9120.0 },  // 12 years
        MaintenanceDataPoint { x: 15.0, y: 12000.0 }, // 15 years
    ];

    db.set(f150);

    db
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Car {
    pub id: usize,
    pub make: String,
    pub model: String,
    pub trim: String,
    pub year: String,
    pub purchase_price: String,
    pub current_mileage: String,
    pub mpg: String,
    pub insurance_cost: String,
    pub vin: String,
    pub listing_url: String,
    pub notes: String,
}

impl Car {
    fn new(id: usize) -> Self {
        Self {
            id,
            make: String::new(),
            model: String::new(),
            trim: String::new(),
            year: String::new(),
            purchase_price: String::new(),
            current_mileage: String::new(),
            mpg: String::new(),
            insurance_cost: String::new(),
            vin: String::new(),
            listing_url: String::new(),
            notes: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComputedCarData {
    pub remaining_miles: f64,
    pub years_remaining: f64,
    pub fuel_cost_total: f64,
    pub fuel_cost_annual: f64,
    pub insurance_cost_annual: f64,
    pub maintenance_cost_total: f64,
    pub maintenance_cost_annual: f64,
    pub opportunity_cost: f64,
    pub total_cost_of_ownership: f64,
    pub annual_cost: f64,
}

// Compute all derived fields from user inputs and shared settings
fn compute_car_data(
    car: &Car,
    settings: &SharedSettings,
    maintenance_db: &MaintenanceCostDatabase,
) -> Option<ComputedCarData> {
    // Parse required user inputs
    let purchase_price = car.purchase_price.parse::<f64>().ok()?;
    let current_mileage = car.current_mileage.parse::<f64>().ok()?;
    let mpg = car.mpg.parse::<f64>().ok()?;
    let insurance_cost_6month = car.insurance_cost.parse::<f64>().ok()?;

    // Validate inputs
    if mpg <= 0.0 || settings.annual_mileage <= 0.0 {
        return None;
    }

    // Step 1: Calculate remaining miles
    let remaining_miles = settings.lifetime_miles - current_mileage;
    if remaining_miles <= 0.0 {
        return None;
    }

    // Step 2: Calculate years remaining
    let years_remaining = remaining_miles / settings.annual_mileage;

    // Step 3: Calculate fuel costs
    let fuel_cost_total = (remaining_miles / mpg) * settings.average_gas_price;
    let fuel_cost_annual = fuel_cost_total / years_remaining;

    // Step 4: Calculate insurance costs
    let insurance_cost_annual = insurance_cost_6month * 2.0;

    // Step 5: Calculate maintenance costs
    // Split 50/50 between mileage-based and time-based costs
    let maintenance_cost_total = if let Some(maint_data) =
        maintenance_db.get(&car.make, &car.model)
    {
        let end_miles = current_mileage + remaining_miles;
        let mileage_cost = maint_data.cost_for_mileage_range(current_mileage, end_miles);

        // Calculate current age and end age of vehicle
        // We need to estimate the vehicle's current age based on mileage
        let current_age = current_mileage / settings.annual_mileage;
        let end_age = current_age + years_remaining;
        let time_cost = maint_data.cost_for_time_range(current_age, end_age);

        // Average the two costs (50/50 split)
        (mileage_cost + time_cost) / 2.0
    } else {
        0.0
    };
    let maintenance_cost_annual = maintenance_cost_total / years_remaining;

    // Step 6: Calculate opportunity cost
    let opportunity_cost = purchase_price * (settings.opportunity_cost_rate / 100.0) * years_remaining;

    // Step 7: Calculate total cost of ownership
    let total_cost_of_ownership = purchase_price
        + fuel_cost_total
        + maintenance_cost_total
        + (insurance_cost_annual * years_remaining)
        + opportunity_cost;

    // Step 8: Calculate annual cost
    let annual_cost = total_cost_of_ownership / years_remaining;

    Some(ComputedCarData {
        remaining_miles,
        years_remaining,
        fuel_cost_total,
        fuel_cost_annual,
        insurance_cost_annual,
        maintenance_cost_total,
        maintenance_cost_annual,
        opportunity_cost,
        total_cost_of_ownership,
        annual_cost,
    })
}

// Components

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/carcalc.css"/>
        <Title text="CarCalc - Total Cost of Ownership Calculator"/>
        <Meta name="description" content="Calculate the true total cost of owning any car"/>

        <Router>
            <div class="min-h-screen bg-gray-50">
                <header class="bg-white shadow">
                    <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                        <h1 class="text-3xl font-bold text-gray-900">"CarCalc"</h1>
                        <p class="mt-1 text-sm text-gray-600">"Calculate the true cost of car ownership"</p>
                    </div>
                </header>

                <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    <Routes fallback=|| view! { <p>"Page not found"</p> }>
                        <Route path=StaticSegment("/") view=HomePage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn SharedSettingsForm(
    settings: Signal<SharedSettings>,
    set_settings: WriteSignal<SharedSettings>,
) -> impl IntoView {
    view! {
        <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <h2 class="text-xl font-semibold text-gray-900 mb-4">
                    "Shared Settings"
                </h2>
                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
                    <div>
                        <label for="opportunity-rate" class="block text-sm font-medium text-gray-700">
                            "Opportunity Cost Rate (%)"
                        </label>
                        <input
                            type="number"
                            step="0.1"
                            id="opportunity-rate"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            prop:value=move || settings.get().opportunity_cost_rate
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(8.0);
                                set_settings.update(|s| s.opportunity_cost_rate = value);
                            }
                        />
                    </div>
                    <div>
                        <label for="annual-mileage" class="block text-sm font-medium text-gray-700">
                            "Annual Mileage"
                        </label>
                        <input
                            type="number"
                            step="1000"
                            id="annual-mileage"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            prop:value=move || settings.get().annual_mileage
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(12000.0);
                                set_settings.update(|s| s.annual_mileage = value);
                            }
                        />
                    </div>
                    <div>
                        <label for="lifetime-miles" class="block text-sm font-medium text-gray-700">
                            "Default Lifetime Miles"
                        </label>
                        <input
                            type="number"
                            step="10000"
                            id="lifetime-miles"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            prop:value=move || settings.get().lifetime_miles
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(200000.0);
                                set_settings.update(|s| s.lifetime_miles = value);
                            }
                        />
                    </div>
                    <div>
                        <label for="gas-price" class="block text-sm font-medium text-gray-700">
                            "Average Gas Price ($/gallon)"
                        </label>
                        <input
                            type="number"
                            step="0.01"
                            id="gas-price"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            prop:value=move || settings.get().average_gas_price
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(3.50);
                                set_settings.update(|s| s.average_gas_price = value);
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CarCostSummary(computed: ComputedCarData) -> impl IntoView {
    view! {
        <div class="mt-6 border-t border-gray-200 pt-6">
            <h3 class="text-lg font-semibold text-gray-900 mb-4">"Calculated Costs"</h3>

            <div class="bg-blue-50 rounded-lg p-4 mb-4">
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <div class="text-sm font-medium text-gray-600">"Total Cost of Ownership"</div>
                        <div class="text-2xl font-bold text-blue-600">
                            {format!("${:.2}", computed.total_cost_of_ownership)}
                        </div>
                    </div>
                    <div>
                        <div class="text-sm font-medium text-gray-600">"Annual Cost"</div>
                        <div class="text-2xl font-bold text-blue-600">
                            {format!("${:.2}", computed.annual_cost)}
                        </div>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Years Remaining"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("{:.1}", computed.years_remaining)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Remaining Miles"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("{:.0}", computed.remaining_miles)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Fuel Cost (Total)"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("${:.2}", computed.fuel_cost_total)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Fuel Cost (Annual)"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("${:.2}", computed.fuel_cost_annual)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Insurance (Annual)"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("${:.2}", computed.insurance_cost_annual)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Opportunity Cost"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("${:.2}", computed.opportunity_cost)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Maintenance (Total)"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("${:.2}", computed.maintenance_cost_total)}
                    </div>
                </div>

                <div class="bg-white p-3 rounded border border-gray-200">
                    <div class="text-xs text-gray-500 uppercase tracking-wide">"Maintenance (Annual)"</div>
                    <div class="text-lg font-semibold text-gray-900 mt-1">
                        {format!("${:.2}", computed.maintenance_cost_annual)}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CarForm(
    car: ReadSignal<Car>,
    set_car_wrapper: impl Fn(&dyn Fn(&mut Car)) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="mt-4 space-y-6">
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Make"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().make
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.make = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Model"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().model
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.model = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Trim/Features (optional)"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().trim
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.trim = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Model Year"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().year
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.year = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        "Purchase Price ($)"
                        <span class="text-red-600">" *"</span>
                    </label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().purchase_price
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.purchase_price = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        "Current Mileage"
                        <span class="text-red-600">" *"</span>
                    </label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().current_mileage
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.current_mileage = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        "MPG"
                        <span class="text-red-600">" *"</span>
                    </label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().mpg
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.mpg = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        "Insurance Cost (6-month premium $)"
                        <span class="text-red-600">" *"</span>
                    </label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().insurance_cost
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.insurance_cost = event_target_value(&ev));
                        }
                    />
                </div>
            </div>

            <div class="border-t border-gray-200 pt-6">
                <h4 class="text-sm font-medium text-gray-900 mb-4">"Additional Information"</h4>
                <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">"VIN (optional)"</label>
                        <input
                            type="text"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            prop:value=move || car.get().vin
                            on:input=move |ev| {
                                set_car_wrapper(&|c| c.vin = event_target_value(&ev));
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">"Listing URL (optional)"</label>
                        <input
                            type="text"
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            prop:value=move || car.get().listing_url
                            on:input=move |ev| {
                                set_car_wrapper(&|c| c.listing_url = event_target_value(&ev));
                            }
                        />
                    </div>
                </div>
                <div class="mt-6">
                    <label class="block text-sm font-medium text-gray-700">"Notes (optional)"</label>
                    <textarea
                        rows="3"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().notes
                        on:input=move |ev| {
                            set_car_wrapper(&|c| c.notes = event_target_value(&ev));
                        }
                    ></textarea>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CarCard(
    car: Car,
    update_car: impl Fn(Car) + 'static + Copy + Send + Sync,
    car_id: usize,
    expanded_cars: ReadSignal<Vec<usize>>,
    set_expanded_cars: WriteSignal<Vec<usize>>,
    settings: Signal<SharedSettings>,
    maintenance_db: Signal<MaintenanceCostDatabase>,
    on_delete: Box<dyn Fn()>,
) -> impl IntoView {
    let (car_signal, set_car_signal) = signal(car);

    // Create a wrapper that updates both local signal and parent
    let set_car_wrapper = move |f: &dyn Fn(&mut Car)| {
        set_car_signal.update(f);
        update_car(car_signal.get());
    };

    let is_expanded = move || expanded_cars.get().contains(&car_id);

    let toggle_expanded = move |_| {
        set_expanded_cars.update(|expanded| {
            if expanded.contains(&car_id) {
                expanded.retain(|&id| id != car_id);
            } else {
                expanded.push(car_id);
            }
        });
    };

    let car_display = move || {
        let c = car_signal.get();
        let name = if !c.make.is_empty() || !c.model.is_empty() {
            format!("{} {}", c.make, c.model).trim().to_string()
        } else {
            format!("Car #{}", c.id)
        };
        let year = if !c.year.is_empty() {
            format!(" ({})", c.year)
        } else {
            String::new()
        };
        format!("{}{}", name, year)
    };

    let computed_data = move || compute_car_data(&car_signal.get(), &settings.get(), &maintenance_db.get());

    view! {
        <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <div class="flex items-center justify-between">
                    <button
                        class="flex-1 flex items-center text-left"
                        on:click=toggle_expanded
                    >
                        <span class="text-lg font-medium text-gray-900">{car_display}</span>
                        <svg
                            class=move || format!(
                                "ml-2 h-5 w-5 transform transition-transform {}",
                                if is_expanded() { "rotate-180" } else { "" }
                            )
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"/>
                        </svg>
                    </button>
                    <button
                        class="ml-4 text-red-600 hover:text-red-800"
                        on:click=move |_| on_delete()
                    >
                        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd"/>
                        </svg>
                    </button>
                </div>

                <Show when=is_expanded>
                    <CarForm car=car_signal set_car_wrapper=set_car_wrapper />
                    {move || {
                        if let Some(computed) = computed_data() {
                            view! { <CarCostSummary computed=computed /> }.into_any()
                        } else {
                            view! {
                                <div class="mt-6 border-t border-gray-200 pt-6">
                                    <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
                                        <div class="flex">
                                            <svg class="h-5 w-5 text-yellow-400 mr-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                                                <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                                            </svg>
                                            <div>
                                                <h4 class="text-sm font-medium text-yellow-800">"Missing required information"</h4>
                                                <p class="mt-1 text-sm text-yellow-700">
                                                    "Please fill in all required fields (marked with "
                                                    <span class="text-red-600">"*"</span>
                                                    ") to calculate costs."
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}

#[component]
fn MaintenanceDataEditor(
    maintenance_db: Signal<MaintenanceCostDatabase>,
    _set_maintenance_db: WriteSignal<MaintenanceCostDatabase>,
) -> impl IntoView {
    let (selected_key, set_selected_key) = signal::<Option<String>>(None);
    let (is_expanded, set_is_expanded) = signal(false);

    let all_makes_models = move || {
        maintenance_db.get().get_all_keys()
    };

    let selected_data = move || {
        if let Some(key) = selected_key.get() {
            let parts: Vec<&str> = key.split('_').collect();
            if parts.len() >= 2 {
                let make = parts[0];
                let model = parts[1..].join("_");
                return maintenance_db.get().get(make, &model).cloned();
            }
        }
        None
    };

    view! {
        <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <div class="flex items-center justify-between">
                    <div class="flex-1">
                        <h2 class="text-xl font-semibold text-gray-900">"Maintenance Cost Data"</h2>
                        <p class="mt-1 text-sm text-gray-600">
                            "View and edit maintenance cost tables per make/model. Data is shared across all cars of the same type."
                        </p>
                    </div>
                    <button
                        class="ml-4 text-gray-600 hover:text-gray-800"
                        on:click=move |_| set_is_expanded.update(|v| *v = !*v)
                    >
                        <svg
                            class=move || format!(
                                "h-6 w-6 transform transition-transform {}",
                                if is_expanded.get() { "rotate-180" } else { "" }
                            )
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                        >
                            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"/>
                        </svg>
                    </button>
                </div>

                <Show when=move || is_expanded.get()>
                    <div class="mt-6 space-y-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                "Select Make/Model"
                            </label>
                            <select
                                class="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_selected_key.set(if value.is_empty() { None } else { Some(value) });
                                }
                            >
                                <option value="">"-- Select a vehicle --"</option>
                                <For
                                    each=all_makes_models
                                    key=|(make, model)| format!("{}_{}", make, model)
                                    children=move |(make, model)| {
                                        let key = format!("{}_{}", make.to_lowercase(), model.to_lowercase());
                                        view! {
                                            <option value=key>
                                                {format!("{} {}", make, model)}
                                            </option>
                                        }
                                    }
                                />
                            </select>
                        </div>

                        <Show when=move || selected_data().is_some()>
                            {move || {
                                if let Some(data) = selected_data() {
                                    view! {
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                                            <div class="border border-gray-200 rounded-lg p-4">
                                                <h3 class="text-lg font-semibold text-gray-900 mb-2">
                                                    "By Mileage"
                                                </h3>
                                                <p class="text-xs text-gray-500 mb-3">
                                                    "Cumulative cost per 10k miles"
                                                </p>
                                                <div class="space-y-2 max-h-96 overflow-y-auto">
                                                    <For
                                                        each=move || data.by_mileage.clone()
                                                        key=|point| format!("{}", point.x)
                                                        children=move |point| {
                                                            view! {
                                                                <div class="flex items-center space-x-2 text-sm">
                                                                    <span class="w-20 text-gray-600">
                                                                        {format!("{}k mi", point.x * 10.0)}
                                                                    </span>
                                                                    <span class="flex-1 text-gray-900">
                                                                        {format!("${:.2}", point.y)}
                                                                    </span>
                                                                </div>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            </div>

                                            <div class="border border-gray-200 rounded-lg p-4">
                                                <h3 class="text-lg font-semibold text-gray-900 mb-2">
                                                    "By Time"
                                                </h3>
                                                <p class="text-xs text-gray-500 mb-3">
                                                    "Cumulative cost per year"
                                                </p>
                                                <div class="space-y-2 max-h-96 overflow-y-auto">
                                                    <For
                                                        each=move || data.by_time.clone()
                                                        key=|point| format!("{}", point.x)
                                                        children=move |point| {
                                                            view! {
                                                                <div class="flex items-center space-x-2 text-sm">
                                                                    <span class="w-20 text-gray-600">
                                                                        {format!("{} yr", point.x)}
                                                                    </span>
                                                                    <span class="flex-1 text-gray-900">
                                                                        {format!("${:.2}", point.y)}
                                                                    </span>
                                                                </div>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            </div>
                                        </div>

                                        <div class="mt-4 bg-blue-50 border border-blue-200 rounded-lg p-4">
                                            <div class="flex">
                                                <svg class="h-5 w-5 text-blue-400 mr-3" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                                                </svg>
                                                <div class="flex-1">
                                                    <h4 class="text-sm font-medium text-blue-800">"How to update this data"</h4>
                                                    <p class="mt-1 text-sm text-blue-700">
                                                        "This data comes from CarEdge.com. To update it, visit CarEdge, find your vehicle's maintenance costs, and manually enter the data here. Data is stored locally in your browser."
                                                    </p>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <div></div> }.into_any()
                                }
                            }}
                        </Show>
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn CarList(
    cars: Signal<Vec<Car>>,
    set_cars: WriteSignal<Vec<Car>>,
    settings: Signal<SharedSettings>,
    maintenance_db: Signal<MaintenanceCostDatabase>,
) -> impl IntoView {
    let (expanded_cars, set_expanded_cars) = signal(Vec::<usize>::new());
    let next_id = RwSignal::new(1_usize);

    // Initialize next_id from existing cars
    if let Some(max_id) = cars.get().iter().map(|c| c.id).max() {
        next_id.set(max_id + 1);
    }

    let add_car = move |_| {
        let id = next_id.get();
        next_id.update(|n| *n += 1);

        let new_car = Car::new(id);
        set_cars.update(|cars| {
            cars.push(new_car);
        });
        set_expanded_cars.update(|expanded| {
            expanded.push(id);
        });
    };

    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <h2 class="text-xl font-semibold text-gray-900">"Your Cars"</h2>
                <button
                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                    on:click=add_car
                >
                    <svg class="mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd"/>
                    </svg>
                    "Add Car"
                </button>
            </div>

            <For
                each=move || cars.get().into_iter().enumerate()
                key=|(_, car)| car.id
                children=move |(index, car)| {
                    let car_id = car.id;

                    let update_car = {
                        let set_cars = set_cars.clone();
                        move |updated_car: Car| {
                            set_cars.update(|cars| {
                                if index < cars.len() {
                                    cars[index] = updated_car;
                                }
                            });
                        }
                    };

                    let on_delete = {
                        let set_cars = set_cars.clone();
                        let set_expanded_cars = set_expanded_cars.clone();
                        move || {
                            set_cars.update(|cars| {
                                cars.retain(|c| c.id != car_id);
                            });
                            set_expanded_cars.update(|expanded| {
                                expanded.retain(|&id| id != car_id);
                            });
                        }
                    };

                    view! {
                        <CarCard
                            car=car
                            update_car=update_car
                            car_id=car_id
                            expanded_cars=expanded_cars
                            set_expanded_cars=set_expanded_cars
                            settings=settings
                            maintenance_db=maintenance_db
                            on_delete=Box::new(on_delete)
                        />
                    }
                }
            />

            <Show when=move || cars.get().is_empty()>
                <div class="text-center py-12 bg-white rounded-lg shadow">
                    <svg class="mx-auto h-12 w-12 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                    </svg>
                    <h3 class="mt-2 text-sm font-medium text-gray-900">"No cars yet"</h3>
                    <p class="mt-1 text-sm text-gray-500">"Get started by adding a car to compare."</p>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (settings, set_settings, _) =
        use_local_storage::<SharedSettings, JsonSerdeCodec>("carcalc_settings");

    let (maintenance_db, set_maintenance_db, _) =
        use_local_storage::<MaintenanceCostDatabase, JsonSerdeCodec>("carcalc_maintenance_db");

    let (cars, set_cars, _) =
        use_local_storage::<Vec<Car>, JsonSerdeCodec>("carcalc_cars");

    view! {
        <div class="px-4 py-6 sm:px-0 space-y-6">
            <SharedSettingsForm settings=settings set_settings=set_settings />
            <MaintenanceDataEditor maintenance_db=maintenance_db _set_maintenance_db=set_maintenance_db />
            <CarList cars=cars set_cars=set_cars settings=settings maintenance_db=maintenance_db />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maintenance_data_interpolation() {
        let mut data = MaintenanceCostData::new("Toyota".to_string(), "Prius".to_string());

        // Simple linear data: $100 per 10k miles
        data.by_mileage = vec![
            MaintenanceDataPoint { x: 1.0, y: 100.0 },
            MaintenanceDataPoint { x: 2.0, y: 200.0 },
            MaintenanceDataPoint { x: 3.0, y: 300.0 },
        ];

        // Test exact point
        let cost = data.cost_for_mileage_range(0.0, 10000.0);
        assert!((cost - 100.0).abs() < 0.01, "Expected ~100, got {}", cost);

        // Test interpolation
        let cost = data.cost_for_mileage_range(0.0, 15000.0);
        assert!((cost - 150.0).abs() < 0.01, "Expected ~150, got {}", cost);

        // Test range
        let cost = data.cost_for_mileage_range(10000.0, 20000.0);
        assert!((cost - 100.0).abs() < 0.01, "Expected ~100, got {}", cost);
    }

    #[test]
    fn test_maintenance_data_extrapolation() {
        let mut data = MaintenanceCostData::new("Toyota".to_string(), "Prius".to_string());

        data.by_mileage = vec![
            MaintenanceDataPoint { x: 1.0, y: 100.0 },
            MaintenanceDataPoint { x: 2.0, y: 200.0 },
        ];

        // Test extrapolation beyond last point
        let cost = data.cost_for_mileage_range(0.0, 30000.0);
        assert!(cost > 200.0, "Expected >200, got {}", cost);
    }

    #[test]
    fn test_sample_data_exists() {
        let db = get_sample_maintenance_data();

        // Test that Toyota Prius exists
        let prius = db.get("Toyota", "Prius");
        assert!(prius.is_some(), "Toyota Prius should exist in sample data");

        let prius = prius.unwrap();
        assert!(!prius.by_mileage.is_empty(), "Prius should have mileage data");
        assert!(!prius.by_time.is_empty(), "Prius should have time data");

        // Test that Ford F-150 exists
        let f150 = db.get("Ford", "F-150");
        assert!(f150.is_some(), "Ford F-150 should exist in sample data");

        let f150 = f150.unwrap();
        assert!(!f150.by_mileage.is_empty(), "F-150 should have mileage data");
        assert!(!f150.by_time.is_empty(), "F-150 should have time data");

        // Verify F-150 costs more than Prius (trucks typically cost more to maintain)
        let prius_100k = prius.cost_for_mileage_range(0.0, 100000.0);
        let f150_100k = f150.cost_for_mileage_range(0.0, 100000.0);
        assert!(f150_100k > prius_100k, "F-150 should cost more than Prius to maintain");
    }

    #[test]
    fn test_maintenance_cost_calculation_with_car() {
        let db = get_sample_maintenance_data();
        let settings = SharedSettings::default();

        let mut car = Car::new(1);
        car.make = "Toyota".to_string();
        car.model = "Prius".to_string();
        car.purchase_price = "25000".to_string();
        car.current_mileage = "50000".to_string();
        car.mpg = "50".to_string();
        car.insurance_cost = "500".to_string();

        let computed = compute_car_data(&car, &settings, &db);
        assert!(computed.is_some(), "Should compute data for valid car");

        let computed = computed.unwrap();
        assert!(computed.maintenance_cost_total > 0.0, "Should have maintenance costs");
        assert!(computed.maintenance_cost_annual > 0.0, "Should have annual maintenance costs");

        // Maintenance should be included in total
        assert!(computed.total_cost_of_ownership > computed.fuel_cost_total,
            "TCO should include more than just fuel");
    }

    #[test]
    fn test_maintenance_cost_50_50_split() {
        let mut data = MaintenanceCostData::new("Test".to_string(), "Car".to_string());

        // Set up simple linear data: $100 per 10k miles / per year
        data.by_mileage = vec![
            MaintenanceDataPoint { x: 10.0, y: 1000.0 },
            MaintenanceDataPoint { x: 20.0, y: 2000.0 },
        ];
        data.by_time = vec![
            MaintenanceDataPoint { x: 10.0, y: 1200.0 },
            MaintenanceDataPoint { x: 20.0, y: 2400.0 },
        ];

        let mut db = MaintenanceCostDatabase::default();
        db.set(data);

        let settings = SharedSettings::default(); // 12k miles/year, 200k lifetime

        let mut car = Car::new(1);
        car.make = "Test".to_string();
        car.model = "Car".to_string();
        car.purchase_price = "10000".to_string();
        car.current_mileage = "0".to_string();
        car.mpg = "30".to_string();
        car.insurance_cost = "500".to_string();

        let computed = compute_car_data(&car, &settings, &db);
        assert!(computed.is_some());

        let computed = computed.unwrap();

        // Car will go from 0 to 200k miles (20.0 x units) and 0 to ~16.67 years
        // Mileage cost: 2000 (20.0 * 100 per unit)
        // Time cost: ~2000 (16.67 * 120 per year)
        // Average: 2000

        // Verify it's using both mileage and time (should be ~2000 for 50/50 split)
        assert!(computed.maintenance_cost_total > 1500.0 && computed.maintenance_cost_total < 2500.0,
            "Maintenance cost should be around 2000 for 50/50 split. Got {}",
            computed.maintenance_cost_total);
    }
}
