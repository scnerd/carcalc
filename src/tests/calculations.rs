use crate::calculations::compute_car_data;
use crate::data::get_sample_maintenance_data;
use crate::models::{
    Car, MaintenanceCostData, MaintenanceCostDatabase, MaintenanceDataPoint, SharedSettings,
};

#[test]
fn test_maintenance_data_interpolation() {
    let mut data = MaintenanceCostData::new("Toyota".to_string(), "Prius".to_string());

    // Simple linear data: $100 per 10k miles
    data.by_mileage = vec![
        MaintenanceDataPoint { x: 1.0, y: 100.0 },
        MaintenanceDataPoint { x: 2.0, y: 200.0 },
        MaintenanceDataPoint { x: 3.0, y: 300.0 },
    ];

    // Test exact point
    let cost = data.cost_for_mileage_range(0.0, 10000.0);
    assert!((cost - 100.0).abs() < 0.01, "Expected ~100, got {}", cost);

    // Test interpolation
    let cost = data.cost_for_mileage_range(0.0, 15000.0);
    assert!((cost - 150.0).abs() < 0.01, "Expected ~150, got {}", cost);

    // Test range
    let cost = data.cost_for_mileage_range(10000.0, 20000.0);
    assert!((cost - 100.0).abs() < 0.01, "Expected ~100, got {}", cost);
}

#[test]
fn test_maintenance_data_extrapolation() {
    let mut data = MaintenanceCostData::new("Toyota".to_string(), "Prius".to_string());

    data.by_mileage = vec![
        MaintenanceDataPoint { x: 1.0, y: 100.0 },
        MaintenanceDataPoint { x: 2.0, y: 200.0 },
    ];

    // Test extrapolation beyond last point
    let cost = data.cost_for_mileage_range(0.0, 30000.0);
    assert!(cost > 200.0, "Expected >200, got {}", cost);
}

#[test]
fn test_sample_data_exists() {
    let db = get_sample_maintenance_data();

    // Test that Toyota Prius exists
    let prius = db.get("Toyota", "Prius");
    assert!(prius.is_some(), "Toyota Prius should exist in sample data");

    let prius = prius.unwrap();
    assert!(
        !prius.by_mileage.is_empty(),
        "Prius should have mileage data"
    );
    assert!(!prius.by_time.is_empty(), "Prius should have time data");

    // Test that Ford F-150 exists
    let f150 = db.get("Ford", "F-150");
    assert!(f150.is_some(), "Ford F-150 should exist in sample data");

    let f150 = f150.unwrap();
    assert!(
        !f150.by_mileage.is_empty(),
        "F-150 should have mileage data"
    );
    assert!(!f150.by_time.is_empty(), "F-150 should have time data");

    // Verify F-150 costs more than Prius (trucks typically cost more to maintain)
    let prius_100k = prius.cost_for_mileage_range(0.0, 100000.0);
    let f150_100k = f150.cost_for_mileage_range(0.0, 100000.0);
    assert!(
        f150_100k > prius_100k,
        "F-150 should cost more than Prius to maintain"
    );
}

#[test]
fn test_maintenance_cost_calculation_with_car() {
    let db = get_sample_maintenance_data();
    let settings = SharedSettings::default();

    let mut car = Car::new(1);
    car.make = "Toyota".to_string();
    car.model = "Prius".to_string();
    car.purchase_price = "25000".to_string();
    car.current_mileage = "50000".to_string();
    car.mpg = "50".to_string();
    car.insurance_cost = "500".to_string();

    let computed = compute_car_data(&car, &settings, &db);
    assert!(computed.is_some(), "Should compute data for valid car");

    let computed = computed.unwrap();
    assert!(
        computed.maintenance_cost_total > 0.0,
        "Should have maintenance costs"
    );
    assert!(
        computed.maintenance_cost_annual > 0.0,
        "Should have annual maintenance costs"
    );

    // Maintenance should be included in total
    assert!(
        computed.total_cost_of_ownership > computed.fuel_cost_total,
        "TCO should include more than just fuel"
    );
}

#[test]
fn test_maintenance_cost_50_50_split() {
    let mut data = MaintenanceCostData::new("Test".to_string(), "Car".to_string());

    // Set up simple linear data: $100 per 10k miles / per year
    data.by_mileage = vec![
        MaintenanceDataPoint { x: 10.0, y: 1000.0 },
        MaintenanceDataPoint { x: 20.0, y: 2000.0 },
    ];
    data.by_time = vec![
        MaintenanceDataPoint { x: 10.0, y: 1200.0 },
        MaintenanceDataPoint { x: 20.0, y: 2400.0 },
    ];

    let mut db = MaintenanceCostDatabase::default();
    db.set(data);

    let settings = SharedSettings::default(); // 12k miles/year, 200k lifetime

    let mut car = Car::new(1);
    car.make = "Test".to_string();
    car.model = "Car".to_string();
    car.purchase_price = "10000".to_string();
    car.current_mileage = "0".to_string();
    car.mpg = "30".to_string();
    car.insurance_cost = "500".to_string();

    let computed = compute_car_data(&car, &settings, &db);
    assert!(computed.is_some());

    let computed = computed.unwrap();

    // Car will go from 0 to 200k miles (20.0 x units) and 0 to ~16.67 years
    // Mileage cost: 2000 (20.0 * 100 per unit)
    // Time cost: ~2000 (16.67 * 120 per year)
    // Average: 2000

    // Verify it's using both mileage and time (should be ~2000 for 50/50 split)
    assert!(
        computed.maintenance_cost_total > 1500.0 && computed.maintenance_cost_total < 2500.0,
        "Maintenance cost should be around 2000 for 50/50 split. Got {}",
        computed.maintenance_cost_total
    );
}
