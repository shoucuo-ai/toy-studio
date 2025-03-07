use sycamore::{prelude::*, web::events::MouseEvent};

use crate::components::effect_demo::expensive_computation;

#[component(inline_props)]
fn CountDisplay(count: ReadSignal<i32>) -> View {
    view! {
        p {
            "Signal is "(count)
        }
    }
}

#[component]
pub fn EventDemo() -> View {
    let signal = create_signal(0);
    let _double_signal = create_memo(move || signal.get() * 2);
    let derived = create_memo(move || expensive_computation(signal.get()));
    let handle = move |_event: MouseEvent| {
        signal.set(signal.get() + 1);
    };
    view! {
        div {
            "Event Demo"
        }
        button(on:click=handle) {
            "Click me"
        }
        CountDisplay(count=derived)
        (if signal.get() > 0 {
            view! {
                p { "Signal is greater than 0" }
            }
        } else {
            view!{}
        })
    }
}
