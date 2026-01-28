use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

// Module declarations
mod calculations;
mod components;
mod data;
mod models;

#[cfg(test)]
mod tests;

// Public re-exports
pub use calculations::*;
pub use components::*;
pub use data::*;
pub use models::*;

/// WASM entry point
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
