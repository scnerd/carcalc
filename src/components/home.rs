use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

use crate::components::cars::CarList;
use crate::components::maintenance::MaintenanceDataEditor;
use crate::components::settings::SharedSettingsForm;
use crate::models::{Car, MaintenanceCostDatabase, SharedSettings};

#[component]
pub fn HomePage() -> impl IntoView {
    let (settings, set_settings, _) =
        use_local_storage::<SharedSettings, JsonSerdeCodec>("carcalc_settings");

    let (maintenance_db, set_maintenance_db, _) =
        use_local_storage::<MaintenanceCostDatabase, JsonSerdeCodec>("carcalc_maintenance_db");

    let (cars, set_cars, _) = use_local_storage::<Vec<Car>, JsonSerdeCodec>("carcalc_cars");

    view! {
        <div class="px-4 py-6 sm:px-0 space-y-6">
            <SharedSettingsForm settings=settings set_settings=set_settings />
            <MaintenanceDataEditor maintenance_db=maintenance_db _set_maintenance_db=set_maintenance_db />
            <CarList cars=cars set_cars=set_cars settings=settings maintenance_db=maintenance_db />
        </div>
    }
}
