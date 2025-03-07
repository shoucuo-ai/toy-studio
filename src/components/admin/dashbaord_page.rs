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
