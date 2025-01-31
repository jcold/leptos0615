use std::rc::Rc;

use leptos::*;

/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = create_signal(initial_value);

    let show_msg = create_rw_signal(false);

    let handle_toggle = move |_| show_msg.set(!show_msg.get_untracked());

    view! {
        <div>
            <button on:click=handle_toggle>"Show"</button>
        </div>

        <Show when=move || show_msg.get()>
            <Msg />
        </Show>
    }
}

#[component]
pub fn Msg() -> impl IntoView {
    let val = RcRwSignal::new(0);
    let rc_raw = Rc::new(1);

    {
        let val = val.clone();
        let rc_raw = rc_raw.clone();
        set_interval(
            move || {
                tracing::info!(
                    "rc_strong: {}, rc_raw: {}",
                    val.strong_count(),
                    Rc::strong_count(&rc_raw)
                );
            },
            std::time::Duration::from_secs(1),
        );
    }

    // on_cleanup({
    //     let val = val.clone();
    //     move || {
    //         tracing::info!("cleanup");

    //         set_timeout(
    //             move || {
    //                 tracing::info!("rc_strong: {}", val.strong_count());
    //             },
    //             std::time::Duration::from_secs(3),
    //         );
    //     }
    // });

    let val2 = val.clone();
    let handler = move |_| {
        let _ = rc_raw.clone();
        val2.set(val2.get_untracked() + 1);
    };

    let el_btn = NodeRef::new();
    let stop = leptos_use::use_event_listener(el_btn, ev::click, handler);
    // on_cleanup(move || {
    //     tracing::info!("cleanup event listener");
    //     stop()
    // });

    let stop_callback = store_value(Some(stop));

    let el_clean = NodeRef::new();
    let handler_clean = move |_| {
        tracing::info!("do clean");
        stop_callback.update_value(|v| {
            if let Some(v) = v.take() {
                v();
            }
        });
    };


    view! {
        <div>
            <button node_ref=el_btn >"+1"</button>
            <button node_ref=el_clean on:click=handler_clean >"clean"</button>
            {move || val.get()}
        </div>
    }
}
