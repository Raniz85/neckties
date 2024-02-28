mod app;
mod knot;

use leptos::*;
use app::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub fn main() {

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! {cx, <App/> }
    });
}
