use leptos::prelude::*;

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
