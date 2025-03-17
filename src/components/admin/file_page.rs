use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{common::path_utils, components::{AdminLayout, AdminRoute}, store::AppConfig};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct OpenDirectoryArgs<'a> {
    dir: &'a str,
}

#[component]
pub fn FilePage() -> View {
    let config = use_context::<Signal<AppConfig>>();

    let open_directory = move |path: &str| {
        let path: String = path.to_string();
        console_log!("path: {}", path);
        let project_root_dir = config.get_clone().project_root_dir.clone();
        console_log!("project_root_dir: {}", project_root_dir);
        let full_path = path_utils::join(&project_root_dir, &path);
        console_log!("full_path: {}", full_path);

        spawn_local(async move {
            let args = OpenDirectoryArgs { dir: &full_path };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            let value = invoke("open_directory", args).await;
            console_log!("value: {:?}", value);
        });
    };

    view! {
        AdminLayout(current_route=AdminRoute::File, inner_view= view! {
            div(class="flex space-x-4 border-b border-gray-200") {
                button(class= "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium") {
                    "Files"
                }
            }
            div(class="min-h-screen bg-gray-100 p-4 sm:p-6 lg:p-8") {

                // Directory cards with responsive grid
                div(class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4 auto-rows-fr") {
                    // Root Directory
                    div(
                        class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 flex flex-col justify-between cursor-pointer",
                        on:click=move |_| open_directory("./")
                    ) {
                        div {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Root" }
                            p(class="text-sm text-gray-500") { "root directory" }
                        }

                    }

                    // Models Directory
                    div(
                        class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 flex flex-col justify-between cursor-pointer",
                        on:click=move |_| open_directory("./models")
                    ) {
                        div {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Models" }
                            p(class="text-sm text-gray-500") { "model directory" }
                        }

                    }

                    // Products Directory
                    div(
                        class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 flex flex-col justify-between cursor-pointer",
                        on:click=move |_| open_directory("./apps")
                    ) {
                        div {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Apps" }
                            p(class="text-sm text-gray-500") { "App directory" }
                        }

                    }

                    // Output Directory
                    div(
                        class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 flex flex-col justify-between cursor-pointer",
                        on:click=move |_| open_directory("./output")
                    ) {
                        div {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Output" }
                            p(class="text-sm text-gray-500") { "Generated files" }
                        }

                    }


                    // Cache Directory
                    div(
                        class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 flex flex-col justify-between cursor-pointer",
                        on:click=move |_| open_directory("./.local/cache")
                    ) {
                        div {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Cache" }
                            p(class="text-sm text-gray-500") { "Cache files" }
                        }

                    }

                    // Temp Directory
                    div(
                        class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-4 flex flex-col justify-between cursor-pointer",
                        on:click=move |_| open_directory("./.local/temp")
                    ) {
                        div {
                            h3(class="text-lg font-semibold text-gray-700 mb-2") { "Temp" }
                            p(class="text-sm text-gray-500") { "Temporary files" }
                        }

                    }

                }
            }
        })
    }
}
