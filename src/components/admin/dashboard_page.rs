use sycamore::prelude::*;

use crate::components::{AdminLayout, AdminRoute};

#[component]
pub fn DashboardPage() -> View {
    view! {
        AdminLayout(current_route=AdminRoute::Dashboard, inner_view=view! {
            div(class="flex space-x-4 border-b border-gray-200") {
                button(class= "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium") {
                    "Dashboard"
                }
            }
            div(class="min-h-screen bg-gray-100 p-8") {
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
        })
    }
}
