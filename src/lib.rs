use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};
use serde::{Deserialize, Serialize};

// Data Structures

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    settings: ReadSignal<SharedSettings>,
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
fn CarForm(
    car: ReadSignal<Car>,
    set_car: WriteSignal<Car>,
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
                            set_car.update(|c| c.make = event_target_value(&ev));
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
                            set_car.update(|c| c.model = event_target_value(&ev));
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
                            set_car.update(|c| c.trim = event_target_value(&ev));
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
                            set_car.update(|c| c.year = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Purchase Price ($)"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().purchase_price
                        on:input=move |ev| {
                            set_car.update(|c| c.purchase_price = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Current Mileage"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().current_mileage
                        on:input=move |ev| {
                            set_car.update(|c| c.current_mileage = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"MPG"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().mpg
                        on:input=move |ev| {
                            set_car.update(|c| c.mpg = event_target_value(&ev));
                        }
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">"Insurance Cost (6-month premium $)"</label>
                    <input
                        type="text"
                        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        prop:value=move || car.get().insurance_cost
                        on:input=move |ev| {
                            set_car.update(|c| c.insurance_cost = event_target_value(&ev));
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
                                set_car.update(|c| c.vin = event_target_value(&ev));
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
                                set_car.update(|c| c.listing_url = event_target_value(&ev));
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
                            set_car.update(|c| c.notes = event_target_value(&ev));
                        }
                    ></textarea>
                </div>
            </div>
        </div>
    }
}

#[component]
fn CarCard(
    car: ReadSignal<Car>,
    set_car: WriteSignal<Car>,
    car_id: usize,
    expanded_cars: ReadSignal<Vec<usize>>,
    set_expanded_cars: WriteSignal<Vec<usize>>,
    on_delete: Box<dyn Fn()>,
) -> impl IntoView {
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
        let c = car.get();
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
                    <CarForm car=car set_car=set_car />
                </Show>
            </div>
        </div>
    }
}

#[component]
fn CarList(
    cars: ReadSignal<Vec<(ReadSignal<Car>, WriteSignal<Car>)>>,
    set_cars: WriteSignal<Vec<(ReadSignal<Car>, WriteSignal<Car>)>>,
) -> impl IntoView {
    let (expanded_cars, set_expanded_cars) = signal(Vec::<usize>::new());
    let next_id = RwSignal::new(1_usize);

    let add_car = move |_| {
        let id = next_id.get();
        next_id.update(|n| *n += 1);

        let new_car = signal(Car::new(id));
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
                key=|(_, (car, _))| car.get().id
                children=move |(index, (car, set_car))| {
                    let car_id = car.get().id;

                    let on_delete = {
                        let set_cars = set_cars.clone();
                        let set_expanded_cars = set_expanded_cars.clone();
                        Box::new(move || {
                            set_cars.update(|cars| {
                                cars.remove(index);
                            });
                            set_expanded_cars.update(|expanded| {
                                expanded.retain(|&id| id != car_id);
                            });
                        })
                    };

                    view! {
                        <CarCard
                            car=car
                            set_car=set_car
                            car_id=car_id
                            expanded_cars=expanded_cars
                            set_expanded_cars=set_expanded_cars
                            on_delete=on_delete
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
    let (settings, set_settings) = signal(SharedSettings::default());
    let (cars, set_cars) = signal(Vec::<(ReadSignal<Car>, WriteSignal<Car>)>::new());

    view! {
        <div class="px-4 py-6 sm:px-0 space-y-6">
            <SharedSettingsForm settings=settings set_settings=set_settings />
            <CarList cars=cars set_cars=set_cars />
        </div>
    }
}
