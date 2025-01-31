use counter::SimpleCounter;
use leptos::*;

pub fn main() {
    wasm_tracing::set_as_global_default();

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <SimpleCounter
                initial_value=0
                step=1
            />
        }
    })
}
