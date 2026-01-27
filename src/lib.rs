use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                    <Routes>
                        <Route path="/" view=HomePage/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="px-4 py-6 sm:px-0">
            <div class="bg-white overflow-hidden shadow rounded-lg">
                <div class="px-4 py-5 sm:p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">
                        "Welcome to CarCalc"
                    </h2>
                    <p class="text-gray-600 mb-4">
                        "A comprehensive tool to calculate the total cost of ownership for any vehicle."
                    </p>
                    <div class="bg-blue-50 border-l-4 border-blue-400 p-4">
                        <div class="flex">
                            <div class="ml-3">
                                <p class="text-sm text-blue-700">
                                    "🚧 Under Construction - More features coming soon!"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
