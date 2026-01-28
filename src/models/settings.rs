use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SharedSettings {
    pub opportunity_cost_rate: f64,
    pub annual_mileage: f64,
    pub lifetime_miles: f64,
    pub average_gas_price: f64,
}

impl Default for SharedSettings {
    fn default() -> Self {
        Self {
            opportunity_cost_rate: 8.0,
            annual_mileage: 12000.0,
            lifetime_miles: 200000.0,
            average_gas_price: 3.50,
        }
    }
}
