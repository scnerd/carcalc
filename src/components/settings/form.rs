use leptos::prelude::*;

use crate::components::ui::Tooltip;
use crate::models::SharedSettings;

#[component]
pub fn SharedSettingsForm(
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
                        <label for="opportunity-rate" class="block text-sm font-medium text-gray-700 inline-flex items-center">
                            "Opportunity Cost Rate (%)"
                            <Tooltip text="The annual return rate you could earn by investing the money instead of tying it up in a car. This represents the financial opportunity you're giving up. Common values: 8-10% for stock market average, 5% for conservative investments." />
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
                        <label for="annual-mileage" class="block text-sm font-medium text-gray-700 inline-flex items-center">
                            "Annual Mileage"
                            <Tooltip text="How many miles you expect to drive per year. This affects fuel costs and maintenance schedules. Average values: 12,000-15,000 miles for typical commuters, 20,000+ for high-mileage drivers, 5,000-8,000 for occasional drivers." />
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
                        <label for="lifetime-miles" class="block text-sm font-medium text-gray-700 inline-flex items-center">
                            "Default Lifetime Miles"
                            <Tooltip text="The total miles you plan to drive a car over its lifetime with you. This determines how long you'll own the vehicle and affects total cost calculations. Common values: 150,000-200,000 miles for most vehicles, 250,000+ for highly reliable cars." />
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
                        <label for="gas-price" class="block text-sm font-medium text-gray-700 inline-flex items-center">
                            "Average Gas Price ($/gallon)"
                            <Tooltip text="The average price per gallon of gas in your area. This affects the fuel cost calculation. Check your local gas station prices or use national averages. Consider using a long-term average rather than current prices for more stable comparisons." />
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
