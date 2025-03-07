use sycamore::prelude::*;

#[component]
pub fn DashboardPage() -> View {
    view! {
        div(class="dashboard-page") {
            h1 { "Dashboard" }
            div(class="dashboard-stats") {
                div(class="stat-card") {
                    h3 { "Total Users" }
                    p { "1,234" }
                }
                div(class="stat-card") {
                    h3 { "Active Users" }
                    p { "789" }
                }
                div(class="stat-card") {
                    h3 { "Total Apps" }
                    p { "56" }
                }
            }
        }
    }
}

#[component]
pub fn UVPage() -> View {
    view! {
        div(class="uv-page") {
            h1 { "UV Statistics" }
            div(class="uv-chart") {
                // 这里可以添加图表组件
                p { "UV chart will be displayed here" }
            }
        }
    }
}

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
