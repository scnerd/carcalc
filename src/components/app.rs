use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

use crate::components::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
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
