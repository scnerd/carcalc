use leptos::prelude::*;

use crate::models::ComputedCarData;

#[component]
pub fn CarCostSummary(computed: ComputedCarData) -> impl IntoView {
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
