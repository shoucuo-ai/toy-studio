use sycamore::prelude::*;

#[component]
pub fn TitleBar() -> View {
    let minimize = move |_| {
        println!("minimize");
    };

    let maximize = move |_| {
        println!("maximize");
    };

    let close = move |_| {
        println!("close");
    };

    view! {
        div(class="titlebar") {
            div(class="titlebar-drag-region") {
                span(class="window-title") { "Tauri Studio" }
            }
            div(class="window-controls") {
                button(class="window-control minimize", on:click=minimize) { "—" }
                button(class="window-control maximize", on:click=maximize) { "□" }
                button(class="window-control close", on:click=close) { "×" }
            }
        }
    }
}
