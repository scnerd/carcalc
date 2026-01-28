use leptos::prelude::*;

use crate::models::Car;

#[component]
pub fn CarForm(
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
