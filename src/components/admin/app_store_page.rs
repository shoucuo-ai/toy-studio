use sycamore::prelude::*;

#[component]
pub fn AppStorePage() -> View {
    view! {
        div(class="min-h-screen bg-gray-100 p-8") {
            h1(class="text-3xl font-bold text-gray-800 mb-8") { "App Store Management" }
            div(class="grid") {
                div(class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition duration-200") {
                    h3(class="text-xl font-semibold text-gray-700 mb-2") { "App Name" }
                    p(class="text-gray-600 mb-4") { "Description of the app" }
                    div(class="flex space-x-3") {
                        button(class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition duration-200") { "Edit" }
                        button(class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition duration-200") { "Delete" }
                    }
                }
            }
        }
    }
}