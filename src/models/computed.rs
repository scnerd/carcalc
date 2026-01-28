#[derive(Clone, Debug)]
pub struct ComputedCarData {
    pub remaining_miles: f64,
    pub years_remaining: f64,
    pub fuel_cost_total: f64,
    pub fuel_cost_annual: f64,
    pub insurance_cost_annual: f64,
    pub maintenance_cost_total: f64,
    pub maintenance_cost_annual: f64,
    pub opportunity_cost: f64,
    pub total_cost_of_ownership: f64,
    pub annual_cost: f64,
}
