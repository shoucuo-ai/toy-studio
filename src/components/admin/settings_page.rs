use sycamore::prelude::*;

#[component]
pub fn SettingsPage() -> View  {
    view! {
        div(class="min-h-screen bg-gray-50 py-8 px-4 sm:px-6 lg:px-8") {
            div(class="max-w-2xl mx-auto") {
                h1(class="text-3xl font-bold text-gray-900 mb-8 text-center") {
                    "System Settings"
                }
                form(class="space-y-6 bg-white shadow rounded-lg p-6") {
                    div(class="space-y-4") {
                        // Language settings
                        div(class="flex flex-col") {
                            label(class="block text-sm font-medium text-gray-700 mb-1") {
                                "Language"
                            }
                            select(
                                class="mt-1 block w-full pl-3 pr-10 py-2 text-base border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 rounded-md"
                            ) {
                                option(value="zh") { "Chinese" }
                                option(value="en") { "English" }
                            }
                        }

                        // Project root directory
                        div(class="flex flex-col") {
                            label(class="block text-sm font-medium text-gray-700 mb-1") {
                                "Project Root Directory"
                            }
                            div(class="flex") {
                                input(
                                    class="flex-1 appearance-none block w-full px-3 py-2 border border-gray-300 rounded-l-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                                    r#type="text",
                                    placeholder="Select project root directory"
                                )
                                button(
                                    class="inline-flex items-center px-4 py-2 border border-l-0 border-gray-300 rounded-r-md bg-gray-50 text-sm font-medium text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500",
                                    r#type="button"
                                ) {
                                    "Browse..."
                                }
                            }
                        }

                        // External UV settings
                        div(class="flex items-center") {
                            input(
                                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                                r#type="checkbox",
                                id="external-uv"
                            )
                            label(
                                class="ml-2 block text-sm text-gray-700",
                                r#for="external-uv"
                            ) {
                                "Enable External UV"
                            }
                        }
                    }

                    // Save button
                    div(class="flex justify-end space-x-3 mt-6") {
                        button(
                            class="inline-flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                            r#type="submit"
                        ) {
                            "Save Changes"
                        }
                        button(
                            class="inline-flex justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                            r#type="button"
                        ) {
                            "Reset"
                        }
                    }
                }
            }
        }
    }
}
