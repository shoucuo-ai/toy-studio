use sycamore::prelude::*;

#[component]
pub fn SettingsPage() -> View {
    view! {
        div(class="min-h-screen bg-gray-50 py-8 px-4 sm:px-6 lg:px-8") {
            div(class="max-w-2xl mx-auto") {
                h1(class="text-3xl font-bold text-gray-900 mb-8 text-center") {
                    "System Settings"
                }
                form(class="space-y-6 bg-white shadow rounded-lg p-6") {
                    div(class="space-y-4") {
                        div(class="flex flex-col") {
                            label(class="block text-sm font-medium text-gray-700 mb-1") {
                                "Site Name"
                            }
                            input(
                                class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                                r#type="text",
                                placeholder="Enter site name"
                            )
                        }
                        div(class="flex flex-col") {
                            label(class="block text-sm font-medium text-gray-700 mb-1") {
                                "Email"
                            }
                            input(
                                class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                                r#type="email",
                                placeholder="Admin email"
                            )
                        }
                    }
                    button(
                        class="mt-6 w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                        r#type="submit"
                    ) {
                        "Save Changes"
                    }
                }
            }
        }
    }
}
