use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::components::{AdminLayout, AdminRoute};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Product {
    id: String,
    name: String,
    version: String,
    description: String,
    icon: String,
    cover_image: String,
    package_type: String,
    introduction: String,
    service_notes: String,
    platforms: Vec<String>,
    category: String,
    status: String,
    created_at: String,
    updated_at: String,
    device_support: DeviceSupport,
    requirements: Requirements,
    download: Download,
    command: Command,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct DeviceSupport {
    cpu: bool,
    nvidia: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Requirements {
    ram: String,
    vram: String,
    disk_space: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Download {
    git_url: String,
    branch: String,
    setup_instructions: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Command {
    start: String,
}

#[derive(Serialize, Deserialize)]
struct SetupArgs {
    product_file: String,
}

#[component]
pub fn AppStorePage() -> View {
    let apps = create_signal(Vec::<Product>::new());
    let show_modal = create_signal(false);
    let selected_app = create_signal(None::<Product>);

    spawn_local({
        let apps = apps.clone();
        async move {
            let result = invoke("get_product_list", JsValue::NULL).await;
            match result.as_string() {
                Some(raw) => {
                    let arr: Result<Vec<Product>, serde_json::Error> = serde_json::from_str(&raw);
                    match arr {
                        Ok(items) => {
                            apps.set(items);
                        }
                        Err(e) => {
                            console_log!("error: {:?}", e);
                        }
                    }
                }
                None => {
                    console_log!("Failed to get product list");
                }
            }
        }
    });

    let open_app_info = move |app: Product| {
        selected_app.set(Some(app));
        show_modal.set(true);
    };

    let close_modal = move |_: web_sys::MouseEvent| {
        show_modal.set(false);
    };

    let handle_install = move |app_id: String| {
        let args = SetupArgs {
            product_file: app_id,
        };
        spawn_local(async move {
            match invoke("product_setup", to_value(&args).unwrap()).await {
                result => match from_value::<Result<String, String>>(result) {
                    Ok(Ok(response)) => {
                        console_log!("Git clone success: {}", response);
                    }
                    Ok(Err(err)) => {
                        console_log!("Git clone failed: {}", err);
                    }
                    Err(e) => {
                        console_log!("Failed to parse response: {:?}", e);
                    }
                },
            }
        });
    };

    view! {
        AdminLayout(current_route=AdminRoute::AppStore, inner_view= view! {
            div(class="flex space-x-4 border-b border-gray-200") {
                button(class= "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium") {
                    "App Store"
                }
            }
            div(class="overflow-x-auto p-4") {
                div(class="grid grid-cols-6 md:grid-cols-6 gap-6") {
                    Keyed(
                        list=apps,
                        key=|app| app.id.clone(),
                        view=move |app| {
                            let app_clone = app.clone();
                            view! {
                                div(class="group relative bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow") {
                                    a(
                                        on:click=move |_| open_app_info(app_clone.clone()),
                                        class="cursor-pointer"
                                    ) {
                                        img(src=app.cover_image,
                                            class="w-full rounded-md bg-gray-200 object-cover group-hover:opacity-75 lg:aspect-auto lg:h-40")
                                        div(class="flex flex-row items-center justify-between mt-2") {
                                            h3(class="text-sm text-gray-700") { (app.name) }
                                            p(class="text-sm font-medium text-gray-300") { (app.version) }
                                        }
                                    }
                                }
                            }
                        }
                    )
                }
            }

            // App Info Modal
            div(style=if show_modal.get() { "display: block" } else { "display: none" }, class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50 flex items-center justify-center") {
                div(class="relative mx-auto p-5 border w-11/12 md:w-3/4 lg:w-1/2 shadow-lg rounded-md bg-white") {
                    (if let Some(app) = selected_app.get_clone() {
                        let app_id = app.id.clone();
                        view! {
                            // Modal header
                            div(class="flex justify-between items-center border-b pb-3") {
                                div(class="flex items-center space-x-3") {
                                    img(src=app.icon, class="w-10 h-10 rounded-md")
                                    h3(class="text-xl font-semibold text-gray-700") { (app.name) }
                                    span(class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full") { (app.version) }
                                }
                                button(on:click=close_modal, class="text-gray-400 hover:text-gray-500") {
                                    "×"
                                }
                            }

                            // Modal body
                            div(class="mt-4 space-y-4 max-h-[70vh] overflow-y-auto") {
                                // Description
                                div {
                                    h4(class="text-lg font-medium text-gray-700") { "Description" }
                                    p(class="mt-1 text-gray-600") { (app.description) }
                                }

                                // Introduction
                                div {
                                    h4(class="text-lg font-medium text-gray-700") { "Introduction" }
                                    p(class="mt-1 text-gray-600") { (app.introduction) }
                                }

                                // Service Notes
                                div {
                                    h4(class="text-lg font-medium text-gray-700") { "Service Notes" }
                                    p(class="mt-1 text-gray-600") { (app.service_notes) }
                                }

                                // System Requirements
                                div {
                                    h4(class="text-lg font-medium text-gray-700") { "System Requirements" }
                                    div(class="mt-2 grid grid-cols-2 gap-4") {
                                        div {
                                            p(class="text-sm font-medium text-gray-500") { "RAM" }
                                            p(class="text-gray-600") { (app.requirements.ram) }
                                        }
                                        div {
                                            p(class="text-sm font-medium text-gray-500") { "VRAM" }
                                            p(class="text-gray-600") { (app.requirements.vram) }
                                        }
                                        div {
                                            p(class="text-sm font-medium text-gray-500") { "Disk Space" }
                                            p(class="text-gray-600") { (app.requirements.disk_space) }
                                        }
                                        div {
                                            p(class="text-sm font-medium text-gray-500") { "Device Support" }
                                            p(class="text-gray-600") {
                                                (if app.device_support.cpu { "CPU, " } else { "" })
                                                (if app.device_support.nvidia { "NVIDIA GPU" } else { "" })
                                            }
                                        }
                                    }
                                }

                                // Platforms
                                div {
                                    h4(class="text-lg font-medium text-gray-700") { "Supported Platforms" }
                                    div(class="mt-2 flex flex-wrap gap-2") {
                                        (app.platforms.iter().map(|platform| {
                                            let platform_str = platform.clone();
                                            view! {
                                                span(class="px-2 py-1 bg-gray-100 text-gray-800 text-sm rounded") {
                                                    (platform_str)
                                                }
                                            }
                                        }).collect::<Vec<_>>())
                                    }
                                }

                                // Download Info
                                div {
                                    h4(class="text-lg font-medium text-gray-700") { "Download Information" }
                                    div(class="mt-2") {
                                        p(class="text-sm font-medium text-gray-500") { "Git URL" }
                                        p(class="text-gray-600 break-all") { (app.download.git_url) }

                                        p(class="text-sm font-medium text-gray-500 mt-2") { "Branch" }
                                        p(class="text-gray-600") { (app.download.branch) }

                                        p(class="text-sm font-medium text-gray-500 mt-2") { "Setup Instructions" }
                                        p(class="text-gray-600") { (app.download.setup_instructions) }
                                    }
                                }
                            }

                            // Modal footer
                            div(class="mt-6 flex justify-end space-x-3 border-t pt-3") {
                                button(on:click=close_modal, class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300") {
                                    "Close"
                                }
                                button(
                                    on:click=move |_| {
                                        handle_install(app_id.clone());
                                    },
                                    class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                                ) {
                                    "Install"
                                }
                            }
                        }
                    } else {
                        view! { div { "Loading..." } }
                    })
                }
            }
        })
    }
}
