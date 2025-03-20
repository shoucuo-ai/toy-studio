use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
    common::invoke_tauri,
    components::{AdminLayout, AdminRoute},
};

#[derive(Serialize, Deserialize)]
struct OpenDirectoryArgs {
    dir: String,
}

#[component]
pub fn FilePage() -> View {
    let active_tab = create_signal(0); // 0 表示 Managed Files, 1 表示 System Files

    let open_directory = move |path: &str, managed: bool| {
        let dir = path.to_string();
        spawn_local(async move {
            let command = if managed {
                "open_managed_directory"
            } else {
                "open_system_directory"
            };
            let args = OpenDirectoryArgs { dir: dir };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            let value = invoke_tauri(command, args).await;
            console_log!("value: {:?}", value);
        });
    };

    view! {
        AdminLayout(current_route=AdminRoute::File, inner_view= view! {
            div(class="flex space-x-4 border-b border-gray-200") {
                button(
                    class=move || {
                        if active_tab.get_clone() == 0 {
                            "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                        } else {
                            "px-4 py-2 text-gray-600 hover:text-blue-600 hover:border-b-2 hover:border-blue-600"
                        }
                    },
                    on:click=move |_| active_tab.set(0)
                ) {
                    "Managed Files"
                }
                button(
                    class=move || {
                        if active_tab.get_clone() == 1 {
                            "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                        } else {
                            "px-4 py-2 text-gray-600 hover:text-blue-600 hover:border-b-2 hover:border-blue-600"
                        }
                    },
                    on:click=move |_| active_tab.set(1)
                ) {
                    "System Files"
                }
            }
            div(class="p-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4 auto-rows-fr") {                    // Root Directory
                (if active_tab.get_clone() == 0 {
                    // Managed Files Tab Content
                    view! {
                        // Directory cards with responsive grid
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Root" }
                            p(class="text-sm text-gray-500") { "root directory" }
                        }

                        // Models Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./models", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Models" }
                            p(class="text-sm text-gray-500") { "model directory" }
                        }

                        // Output Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./output", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Output" }
                            p(class="text-sm text-gray-500") { "Generated files" }
                        }

                        // App Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./apps", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Apps" }
                            p(class="text-sm text-gray-500") { "Installed apps directory" }
                        }

                        // Products Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./.local/products", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Products" }
                            p(class="text-sm text-gray-500") { "Uninstalled products directory" }
                        }

                        // Plugins Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./plugins", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Plugins" }
                            p(class="text-sm text-gray-500") { "Plugins directory" }
                        }
                        // Cache Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./.local/cache", true)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Cache" }
                            p(class="text-sm text-gray-500") { "Cache files" }
                        }

                        // Temp Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("./.local/temp", true  )
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Temp" }
                            p(class="text-sm text-gray-500") { "Temporary files" }
                        }
                    }
                } else  if active_tab.get_clone() == 1 {
                    // System Files Tab Content
                    view! {
                        // Data Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("data", false)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Data" }
                            p(class="text-sm text-gray-500") { "System data files" }
                        }

                        // Cache Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("cache", false)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Cache" }
                            p(class="text-sm text-gray-500") { "System cache files" }
                        }

                        // Log Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("log", false)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Log" }
                            p(class="text-sm text-gray-500") { "System log files" }
                        }

                        // Config Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("config", false)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Config" }
                            p(class="text-sm text-gray-500") { "System config files" }
                        }

                        // Local Data Directory
                        div(
                            class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 cursor-pointer",
                            on:click=move |_| open_directory("local_data", false)
                        ) {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Local Data" }
                            p(class="text-sm text-gray-500") { "System local data files" }
                        }
                    }
                } else {
                    view! {
                        "No content"
                    }
                })
            }
        })
    }
}
