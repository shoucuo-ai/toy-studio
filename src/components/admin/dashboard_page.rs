use sycamore::prelude::*;

#[component]
pub fn DashboardPage() -> View {
    view! {
        div(class="min-h-screen bg-gray-100 p-8") {
            h1(class="text-3xl font-bold text-gray-800 mb-8") { "Dashboard" }
            div(class="grid grid-cols-1 md:grid-cols-3 gap-6") {
                div(class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow") {
                    h3(class="text-lg font-semibold text-gray-600 mb-2") { "Total Users" }
                    p(class="text-4xl font-bold text-blue-600") { "1,234" }
                }
                div(class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow") {
                    h3(class="text-lg font-semibold text-gray-600 mb-2") { "Active Users" }
                    p(class="text-4xl font-bold text-green-600") { "789" }
                }
                div(class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow") {
                    h3(class="text-lg font-semibold text-gray-600 mb-2") { "Total Apps" }
                    p(class="text-4xl font-bold text-purple-600") { "56" }
                }
            }
        }
    }
}
