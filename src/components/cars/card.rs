use leptos::prelude::*;

use crate::calculations::compute_car_data;
use crate::components::cars::{CarCostSummary, CarForm};
use crate::models::{Car, MaintenanceCostDatabase, SharedSettings};

#[component]
pub fn CarCard(
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

    let computed_data =
        move || compute_car_data(&car_signal.get(), &settings.get(), &maintenance_db.get());

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
                    {move || {
                        if let Some(computed) = computed_data() {
                            view! {
                                <div class="ml-4 text-right">
                                    <div class="text-sm text-gray-500">"Annual Cost"</div>
                                    <div class="text-lg font-semibold text-blue-600">
                                        {format!("${:.0}", computed.annual_cost)}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
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
