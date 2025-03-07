use sycamore::prelude::*;

#[component]
pub fn SettingsPage() -> View {
    view! {
        div(class="settings-page") {
            h1 { "System Settings" }
            form {
                div(class="form-group") {
                    label { "Site Name" }
                    input(r#type="text", placeholder="Enter site name")
                }
                div(class="form-group") {
                    label { "Email" }
                    input(r#type="email", placeholder="Admin email")
                }
                button(r#type="submit") { "Save Changes" }
            }
        }
    }
}
