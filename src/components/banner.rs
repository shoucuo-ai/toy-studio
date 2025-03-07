use sycamore::prelude::*;

#[component]
pub fn Banner() -> View {
    view! {
        div(class="banner") {
            div(class="banner-left") {
                div(class="logo") {
                    img(src="/tauri.svg", alt="Logo")
                    span(class="system-name") { "Tauri Studio" }
                }
            }

            div(class="banner-center") {
                div(class="search-box") {
                    input(placeholder="search...")
                    span(class="search-icon") { "🔍" }
                }
            }

            div(class="banner-right") {
                div(class="notification") {
                    span(class="notification-icon") { "🔔" }
                    span(class="badge") { "3" }
                }

                div(class="user-profile") {
                    img(src="/avatar-placeholder.png", alt="User Avatar")
                    span(class="username") { "admin" }
                }
            }
        }
    }
}
