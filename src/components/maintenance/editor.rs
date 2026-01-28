use leptos::prelude::*;

use crate::models::MaintenanceCostDatabase;

#[component]
pub fn MaintenanceDataEditor(
    maintenance_db: Signal<MaintenanceCostDatabase>,
    _set_maintenance_db: WriteSignal<MaintenanceCostDatabase>,
) -> impl IntoView {
    let (selected_key, set_selected_key) = signal::<Option<String>>(None);
    let (is_expanded, set_is_expanded) = signal(false);

    let all_makes_models = move || maintenance_db.get().get_all_keys();

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
