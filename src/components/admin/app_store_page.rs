use sycamore::prelude::*;

#[component]
pub fn AppStorePage() -> View {
    view! {
        div(class="appstore-page") {
            h1 { "App Store Management" }
            div(class="app-list") {
                div(class="app-card") {
                    h3 { "App Name" }
                    p { "Description of the app" }
                    button { "Edit" }
                    button { "Delete" }
                }
            }
        }
    }
}