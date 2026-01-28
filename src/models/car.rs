use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Car {
    pub id: usize,
    pub make: String,
    pub model: String,
    pub trim: String,
    pub year: String,
    pub purchase_price: String,
    pub current_mileage: String,
    pub mpg: String,
    pub insurance_cost: String,
    pub vin: String,
    pub listing_url: String,
    pub notes: String,
}

impl Car {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            make: String::new(),
            model: String::new(),
            trim: String::new(),
            year: String::new(),
            purchase_price: String::new(),
            current_mileage: String::new(),
            mpg: String::new(),
            insurance_cost: String::new(),
            vin: String::new(),
            listing_url: String::new(),
            notes: String::new(),
        }
    }
}
