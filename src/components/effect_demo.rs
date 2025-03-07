use sycamore::prelude::*;

pub fn expensive_computation(n: i32) -> i32 {
    // Simulate time-consuming computation: perform some simple math operations
    let mut result = n;
    for i in 0..1000 {
        result = (result + i) % 100;
    }
    result * 3
}

#[component]
pub fn EffectDemo() -> View {
    let signal = create_signal(1);
    let derived = create_memo(move || expensive_computation(signal.get()));
    create_effect(move || {
        let value = signal.get();
        let doubled = derived.get();
        console_log!("value = {value}");
        console_log!("signal = {signal}, doubled = {doubled}");
    });
    view! {
        div {
            "Effect Demo"
        }
        button(on:click=move |_| signal.set(signal.get() + 1)) {
            "Click me"
        }
    }
}
