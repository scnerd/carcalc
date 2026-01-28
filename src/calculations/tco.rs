use crate::models::{Car, ComputedCarData, MaintenanceCostDatabase, SharedSettings};

/// Compute all derived fields from user inputs and shared settings
pub fn compute_car_data(
    car: &Car,
    settings: &SharedSettings,
    maintenance_db: &MaintenanceCostDatabase,
) -> Option<ComputedCarData> {
    // Parse required user inputs
    let purchase_price = car.purchase_price.parse::<f64>().ok()?;
    let current_mileage = car.current_mileage.parse::<f64>().ok()?;
    let mpg = car.mpg.parse::<f64>().ok()?;
    let insurance_cost_6month = car.insurance_cost.parse::<f64>().ok()?;

    // Validate inputs
    if mpg <= 0.0 || settings.annual_mileage <= 0.0 {
        return None;
    }

    // Step 1: Calculate remaining miles
    let remaining_miles = settings.lifetime_miles - current_mileage;
    if remaining_miles <= 0.0 {
        return None;
    }

    // Step 2: Calculate years remaining
    let years_remaining = remaining_miles / settings.annual_mileage;

    // Step 3: Calculate fuel costs
    let fuel_cost_total = (remaining_miles / mpg) * settings.average_gas_price;
    let fuel_cost_annual = fuel_cost_total / years_remaining;

    // Step 4: Calculate insurance costs
    let insurance_cost_annual = insurance_cost_6month * 2.0;

    // Step 5: Calculate maintenance costs
    // Split 50/50 between mileage-based and time-based costs
    let maintenance_cost_total = if let Some(maint_data) = maintenance_db.get(&car.make, &car.model)
    {
        let end_miles = current_mileage + remaining_miles;
        let mileage_cost = maint_data.cost_for_mileage_range(current_mileage, end_miles);

        // Calculate current age and end age of vehicle
        // We need to estimate the vehicle's current age based on mileage
        let current_age = current_mileage / settings.annual_mileage;
        let end_age = current_age + years_remaining;
        let time_cost = maint_data.cost_for_time_range(current_age, end_age);

        // Average the two costs (50/50 split)
        (mileage_cost + time_cost) / 2.0
    } else {
        0.0
    };
    let maintenance_cost_annual = maintenance_cost_total / years_remaining;

    // Step 6: Calculate opportunity cost
    let opportunity_cost =
        purchase_price * (settings.opportunity_cost_rate / 100.0) * years_remaining;

    // Step 7: Calculate total cost of ownership
    let total_cost_of_ownership = purchase_price
        + fuel_cost_total
        + maintenance_cost_total
        + (insurance_cost_annual * years_remaining)
        + opportunity_cost;

    // Step 8: Calculate annual cost
    let annual_cost = total_cost_of_ownership / years_remaining;

    Some(ComputedCarData {
        remaining_miles,
        years_remaining,
        fuel_cost_total,
        fuel_cost_annual,
        insurance_cost_annual,
        maintenance_cost_total,
        maintenance_cost_annual,
        opportunity_cost,
        total_cost_of_ownership,
        annual_cost,
    })
}
