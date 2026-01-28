use crate::models::{MaintenanceCostData, MaintenanceCostDatabase, MaintenanceDataPoint};

/// Sample maintenance cost data based on typical costs for popular vehicles
/// This data represents cumulative maintenance costs over time and mileage
pub fn get_sample_maintenance_data() -> MaintenanceCostDatabase {
    let mut db = MaintenanceCostDatabase::default();

    // Toyota Prius - known for reliability and lower maintenance costs
    // Based on typical maintenance schedules: oil changes, tire rotations, brake service, etc.
    let mut prius = MaintenanceCostData::new("Toyota".to_string(), "Prius".to_string());

    // By mileage (x = 10k miles increments)
    // Cumulative costs include: oil changes, filters, tire rotations, brake pads, fluids, etc.
    prius.by_mileage = vec![
        MaintenanceDataPoint { x: 1.0, y: 350.0 },   // 10k miles
        MaintenanceDataPoint { x: 2.0, y: 700.0 },   // 20k miles
        MaintenanceDataPoint { x: 3.0, y: 1100.0 },  // 30k miles (major service)
        MaintenanceDataPoint { x: 4.0, y: 1450.0 },  // 40k miles
        MaintenanceDataPoint { x: 5.0, y: 1800.0 },  // 50k miles
        MaintenanceDataPoint { x: 6.0, y: 2300.0 },  // 60k miles (major service)
        MaintenanceDataPoint { x: 7.0, y: 2700.0 },  // 70k miles
        MaintenanceDataPoint { x: 8.0, y: 3100.0 },  // 80k miles
        MaintenanceDataPoint { x: 9.0, y: 3650.0 },  // 90k miles (major service)
        MaintenanceDataPoint { x: 10.0, y: 4100.0 }, // 100k miles
        MaintenanceDataPoint { x: 12.0, y: 5200.0 }, // 120k miles (major service)
        MaintenanceDataPoint { x: 15.0, y: 6800.0 }, // 150k miles
        MaintenanceDataPoint { x: 20.0, y: 9500.0 }, // 200k miles
    ];

    // By time (x = years)
    // Split 50/50 with mileage-based costs, assuming 12k miles/year
    prius.by_time = vec![
        MaintenanceDataPoint { x: 1.0, y: 420.0 },   // 1 year
        MaintenanceDataPoint { x: 2.0, y: 840.0 },   // 2 years
        MaintenanceDataPoint { x: 3.0, y: 1320.0 },  // 3 years
        MaintenanceDataPoint { x: 4.0, y: 1740.0 },  // 4 years
        MaintenanceDataPoint { x: 5.0, y: 2160.0 },  // 5 years
        MaintenanceDataPoint { x: 6.0, y: 2760.0 },  // 6 years
        MaintenanceDataPoint { x: 7.0, y: 3240.0 },  // 7 years
        MaintenanceDataPoint { x: 8.0, y: 3720.0 },  // 8 years
        MaintenanceDataPoint { x: 9.0, y: 4380.0 },  // 9 years
        MaintenanceDataPoint { x: 10.0, y: 4920.0 }, // 10 years
        MaintenanceDataPoint { x: 12.0, y: 6240.0 }, // 12 years
        MaintenanceDataPoint { x: 15.0, y: 8160.0 }, // 15 years
    ];

    db.set(prius);

    // Ford F-150 - popular truck with higher maintenance costs
    // Larger engine, more fluids, heavier wear on components
    let mut f150 = MaintenanceCostData::new("Ford".to_string(), "F-150".to_string());

    // By mileage (x = 10k miles increments)
    f150.by_mileage = vec![
        MaintenanceDataPoint { x: 1.0, y: 500.0 },   // 10k miles
        MaintenanceDataPoint { x: 2.0, y: 1000.0 },  // 20k miles
        MaintenanceDataPoint { x: 3.0, y: 1600.0 },  // 30k miles (major service)
        MaintenanceDataPoint { x: 4.0, y: 2150.0 },  // 40k miles
        MaintenanceDataPoint { x: 5.0, y: 2700.0 },  // 50k miles
        MaintenanceDataPoint { x: 6.0, y: 3400.0 },  // 60k miles (major service)
        MaintenanceDataPoint { x: 7.0, y: 4000.0 },  // 70k miles
        MaintenanceDataPoint { x: 8.0, y: 4600.0 },  // 80k miles
        MaintenanceDataPoint { x: 9.0, y: 5350.0 },  // 90k miles (major service)
        MaintenanceDataPoint { x: 10.0, y: 6000.0 }, // 100k miles
        MaintenanceDataPoint { x: 12.0, y: 7600.0 }, // 120k miles (major service)
        MaintenanceDataPoint {
            x: 15.0,
            y: 10000.0,
        }, // 150k miles
        MaintenanceDataPoint {
            x: 20.0,
            y: 14000.0,
        }, // 200k miles
    ];

    // By time (x = years)
    f150.by_time = vec![
        MaintenanceDataPoint { x: 1.0, y: 600.0 },   // 1 year
        MaintenanceDataPoint { x: 2.0, y: 1200.0 },  // 2 years
        MaintenanceDataPoint { x: 3.0, y: 1920.0 },  // 3 years
        MaintenanceDataPoint { x: 4.0, y: 2580.0 },  // 4 years
        MaintenanceDataPoint { x: 5.0, y: 3240.0 },  // 5 years
        MaintenanceDataPoint { x: 6.0, y: 4080.0 },  // 6 years
        MaintenanceDataPoint { x: 7.0, y: 4800.0 },  // 7 years
        MaintenanceDataPoint { x: 8.0, y: 5520.0 },  // 8 years
        MaintenanceDataPoint { x: 9.0, y: 6420.0 },  // 9 years
        MaintenanceDataPoint { x: 10.0, y: 7200.0 }, // 10 years
        MaintenanceDataPoint { x: 12.0, y: 9120.0 }, // 12 years
        MaintenanceDataPoint {
            x: 15.0,
            y: 12000.0,
        }, // 15 years
    ];

    db.set(f150);

    db
}
