use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single data point in a maintenance cost table
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceDataPoint {
    pub x: f64, // mileage (in 10k miles) or years
    pub y: f64, // cumulative cost in dollars
}

/// Maintenance cost data for a specific make+model
/// Contains two tables: one based on mileage, one based on time
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MaintenanceCostData {
    pub make: String,
    pub model: String,
    /// Data points where x = 10k miles, y = cumulative cost over those miles
    pub by_mileage: Vec<MaintenanceDataPoint>,
    /// Data points where x = years, y = cumulative cost over those years
    pub by_time: Vec<MaintenanceDataPoint>,
}

impl MaintenanceCostData {
    pub fn new(make: String, model: String) -> Self {
        Self {
            make,
            model,
            by_mileage: Vec::new(),
            by_time: Vec::new(),
        }
    }

    /// Get a unique key for this make+model combination
    pub fn key(&self) -> String {
        format!("{}_{}", self.make.to_lowercase(), self.model.to_lowercase())
    }

    /// Create a key from make and model strings
    pub fn make_key(make: &str, model: &str) -> String {
        format!("{}_{}", make.to_lowercase(), model.to_lowercase())
    }

    /// Calculate maintenance cost for a given mileage range
    /// Uses linear interpolation between data points
    pub fn cost_for_mileage_range(&self, start_miles: f64, end_miles: f64) -> f64 {
        if self.by_mileage.is_empty() || end_miles <= start_miles {
            return 0.0;
        }

        let start_10k = start_miles / 10000.0;
        let end_10k = end_miles / 10000.0;

        let start_cost = self.interpolate_cost(&self.by_mileage, start_10k);
        let end_cost = self.interpolate_cost(&self.by_mileage, end_10k);

        (end_cost - start_cost).max(0.0)
    }

    /// Calculate maintenance cost for a given time range
    /// Uses linear interpolation between data points
    pub fn cost_for_time_range(&self, start_years: f64, end_years: f64) -> f64 {
        if self.by_time.is_empty() || end_years <= start_years {
            return 0.0;
        }

        let start_cost = self.interpolate_cost(&self.by_time, start_years);
        let end_cost = self.interpolate_cost(&self.by_time, end_years);

        (end_cost - start_cost).max(0.0)
    }

    /// Interpolate cost at a given x value from a series of data points
    fn interpolate_cost(&self, data: &[MaintenanceDataPoint], x: f64) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        // If before first point, extrapolate linearly from origin
        if x <= data[0].x {
            if data[0].x == 0.0 {
                return data[0].y;
            }
            return (data[0].y / data[0].x) * x;
        }

        // If after last point, extrapolate using last two points
        if x >= data[data.len() - 1].x {
            if data.len() == 1 {
                // Only one point, extrapolate from origin
                return (data[0].y / data[0].x) * x;
            }
            let p1 = &data[data.len() - 2];
            let p2 = &data[data.len() - 1];
            let slope = (p2.y - p1.y) / (p2.x - p1.x);
            return p2.y + slope * (x - p2.x);
        }

        // Find the two points to interpolate between
        for i in 0..data.len() - 1 {
            if x >= data[i].x && x <= data[i + 1].x {
                let p1 = &data[i];
                let p2 = &data[i + 1];

                if p2.x == p1.x {
                    return p1.y;
                }

                let ratio = (x - p1.x) / (p2.x - p1.x);
                return p1.y + ratio * (p2.y - p1.y);
            }
        }

        0.0
    }
}

/// Storage for all maintenance cost data, keyed by make_model
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct MaintenanceCostDatabase {
    pub data: HashMap<String, MaintenanceCostData>,
}

impl MaintenanceCostDatabase {
    /// Get maintenance data for a specific make+model
    pub fn get(&self, make: &str, model: &str) -> Option<&MaintenanceCostData> {
        let key = MaintenanceCostData::make_key(make, model);
        self.data.get(&key)
    }

    /// Set maintenance data for a specific make+model
    pub fn set(&mut self, data: MaintenanceCostData) {
        let key = data.key();
        self.data.insert(key, data);
    }

    /// Remove maintenance data for a specific make+model
    pub fn remove(&mut self, make: &str, model: &str) {
        let key = MaintenanceCostData::make_key(make, model);
        self.data.remove(&key);
    }

    /// Get all make+model combinations that have maintenance data
    pub fn get_all_keys(&self) -> Vec<(String, String)> {
        self.data
            .values()
            .map(|d| (d.make.clone(), d.model.clone()))
            .collect()
    }
}
