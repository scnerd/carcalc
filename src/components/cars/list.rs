use leptos::prelude::*;

use crate::components::cars::CarCard;
use crate::models::{Car, MaintenanceCostDatabase, SharedSettings};

#[component]
pub fn CarList(
    cars: Signal<Vec<Car>>,
    set_cars: WriteSignal<Vec<Car>>,
    settings: Signal<SharedSettings>,
    maintenance_db: Signal<MaintenanceCostDatabase>,
) -> impl IntoView {
    let (expanded_cars, set_expanded_cars) = signal(Vec::<usize>::new());
    let next_id = RwSignal::new(1_usize);

    // Initialize next_id from existing cars
    if let Some(max_id) = cars.get_untracked().iter().map(|c| c.id).max() {
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
                        let set_cars = set_cars;
                        move |updated_car: Car| {
                            set_cars.update(|cars| {
                                if index < cars.len() {
                                    cars[index] = updated_car;
                                }
                            });
                        }
                    };

                    let on_delete = {
                        let set_cars = set_cars;
                        let set_expanded_cars = set_expanded_cars;
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
