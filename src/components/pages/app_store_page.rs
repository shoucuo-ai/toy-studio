use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use crate::common::invoke_tauri;
use crate::common::Product;
use crate::common::TabType;
use crate::components::toast::{Toast, ToastNotification, ToastType};
use crate::components::{AdminLayout, AdminRoute};

#[component]
pub fn AppStorePage() -> View {
    let apps = create_signal(Vec::<Product>::new());
    let show_modal = create_signal(false);
    let selected_app = create_signal(None::<Product>);
    let active_tab = create_signal(TabType::Description);
    let toast = create_signal(None::<Toast>);

    spawn_local({
        let apps = apps.clone();
        async move {
            match Product::load_all_products().await {
                Ok(products) => {
                    apps.set(products);
                }
                Err(e) => {
                    toast.set(Some(Toast {
                        message: format!("Failed to load products: {}", e),
                        toast_type: ToastType::Error,
                    }));
                }
            }
        }
    });

    let open_app_info = move |app: Product| {
        selected_app.set(Some(app));
        show_modal.set(true);
        active_tab.set(TabType::Description);
    };

    let close_modal = move |_: web_sys::MouseEvent| {
        show_modal.set(false);
    };

    let handle_install = move |app: Product| {
        let toast = toast.clone();
        spawn_local(async move {
            let json = serde_json::json!({
                "pid": app.id.clone()
            });
            let args = serde_wasm_bindgen::to_value(&json);
            match args {
                Err(e) => {
                    console_log!("Failed to parse response: {:?}", e);
                    toast.set(Some(Toast {
                        message: format!("Failed to parse request: {}", e),
                        toast_type: ToastType::Error,
                    }));
                }
                Ok(args) => {
                    console_log!("args:{:?}", args);
                    let result = invoke_tauri("product_install", args).await;
                    match result.as_string() {
                        None => {
                            console_log!("Failed to get product list");
                            toast.set(Some(Toast {
                                message: "Failed to setup product".to_string(),
                                toast_type: ToastType::Error,
                            }));
                        }
                        Some(raw) => {
                            console_log!("raw:{}", raw);
                            toast.set(Some(Toast {
                                message: "Product installed successfully".to_string(),
                                toast_type: ToastType::Success,
                            }));
                        }
                    }
                }
            }
        });
    };

    view! {
        AdminLayout(current_route=AdminRoute::AppStore, inner_view= view! {
            ToastNotification(toast=toast, duration_ms=3000u32)
            div(class="flex space-x-4 border-b border-gray-200") {
                button(class= "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium") {
                    "App Store"
                }
            }
            div(class="overflow-x-auto p-4") {
                div(class="grid grid-cols-1 xl:grid-cols-6 2xl:grid-cols-6 sm:grid-cols-2 md:grid-cols-4 lg:grid-cols-4 gap-6") {
                    Keyed(
                        list=apps,
                        key=|app| app.id.clone(),
                        view=move |app| {
                            let app_clone = app.clone();
                            view! {
                                div(class="group relative bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow") {
                                    a(class="cursor-pointer", on:click=move |_| open_app_info(app_clone.clone())) {
                                        div(class="relative pb-[56.25%] overflow-hidden rounded-md bg-gray-100") {
                                            div(class="absolute inset-0 flex items-center justify-center bg-gray-100") {
                                                div(class="w-8 h-8 text-gray-300") {
                                                    // Placeholder icon
                                                    svg(xmlns="http://www.w3.org/2000/svg", fill="none", viewBox="0 0 24 24", stroke="currentColor") {
                                                        path(stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z")
                                                    }
                                                }
                                            }
                                            img(src=app.cover_image,
                                                class="absolute inset-0 w-full h-full object-cover group-hover:opacity-75 transition-opacity duration-300 backdrop-blur-sm",
                                                loading="lazy")
                                        }
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
            div(style=if show_modal.get() { "display: block" } else { "display: none" }, class="fixed inset-0 bg-gray-900/40 backdrop-blur-sm overflow-y-auto h-full w-full z-50 flex items-center justify-center") {
                div(class="relative mx-auto p-5 border w-11/12 md:w-3/4 lg:w-1/2 shadow-lg rounded-md bg-white/90 backdrop-blur-md") {
                    (if let Some(app) = selected_app.get_clone() {
                        let app_clone = app.clone();
                        let app_version = app.version.clone();
                        view! {
                            // Modal header
                            div(class="flex justify-between items-center border-b pb-3") {
                                div(class="flex items-center space-x-3") {
                                    img(src=app.icon, class="w-10 h-10 rounded-md")
                                    h3(class="text-xl font-semibold text-gray-700") { (app.name) }
                                    span(class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full") { (app_version) }
                                }
                                button(on:click=close_modal, class="text-gray-400 hover:text-gray-500") {
                                    "×"
                                }
                            }

                            div(class="mt-4 flex flex-row items-center justify-between gap-4") {
                                div(class="w-1/2") {
                                    div(class="relative pb-[56.25%] overflow-hidden rounded-md bg-gray-100") {
                                        div(class="absolute inset-0 flex items-center justify-center bg-gray-100") {
                                            div(class="w-8 h-8 text-gray-300") {
                                                // Placeholder icon
                                                svg(xmlns="http://www.w3.org/2000/svg", fill="none", viewBox="0 0 24 24", stroke="currentColor") {
                                                    path(stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z")
                                                }
                                            }
                                        }
                                        img(src=app.cover_image,
                                            class="absolute inset-0 w-full h-full object-cover rounded-md transition-opacity duration-300",
                                            loading="lazy")
                                    }
                                }
                                div(class="w-1/2") {
                                    // 基础信息表格
                                    div(class="mt-2 border rounded-md overflow-hidden") {
                                        table(class="w-full text-sm") {
                                            tbody {
                                                tr(class="border-b") {
                                                    td(class="px-4 py-2 bg-gray-50 font-medium text-gray-700 border-r") { "Publisher" }
                                                    td(class="px-4 py-2") {
                                                        (match &app.publisher {
                                                            Some(publisher) => publisher.clone(),
                                                            None => "N/A".to_string()
                                                        })
                                                    }
                                                }
                                                tr(class="border-b") {
                                                    td(class="px-4 py-2 bg-gray-50 font-medium text-gray-700 border-r") { "Platforms" }
                                                    td(class="px-4 py-2") {
                                                        div(class="flex flex-wrap gap-1") {
                                                            (app.platforms.iter().map(|platform| {
                                                                let platform_str = platform.clone();
                                                                view! {
                                                                    span(class="px-2 py-0.5 bg-green-100 text-green-800 text-xs rounded") {
                                                                        (platform_str)
                                                                    }
                                                                }
                                                            }).collect::<Vec<_>>())
                                                        }
                                                    }
                                                }
                                                tr(class="border-b") {
                                                    td(class="px-4 py-2 bg-gray-50 font-medium text-gray-700 border-r") { "Device Support" }
                                                    td(class="px-4 py-2") {
                                                        div(class="flex flex-wrap gap-1") {
                                                            (if app.device_support.nvidia {
                                                                view! {
                                                                    span(class="px-2 py-0.5 bg-green-100 text-green-800 text-xs rounded") {
                                                                        "NVIDIA"
                                                                    }
                                                                }
                                                            } else {
                                                                view! { span {} }
                                                            })
                                                            (if app.device_support.cpu {
                                                                view! {
                                                                    span(class="px-2 py-0.5 bg-green-100 text-green-800 text-xs rounded") {
                                                                        "CPU"
                                                                    }
                                                                }
                                                            } else {
                                                                view! { span {} }
                                                            })
                                                        }
                                                    }
                                                }
                                                tr(class="border-b") {
                                                    td(class="px-4 py-2 bg-gray-50 font-medium text-gray-700 border-r") { "Requirements" }
                                                    td(class="px-4 py-2") {
                                                        div(class="flex flex-wrap gap-1") {
                                                            span(class="px-2 py-0.5 bg-green-100 text-green-800 text-xs rounded") {
                                                                (format!("VRAM: {}", app.requirements.vram))
                                                            }
                                                        }
                                                    }
                                                }
                                                tr(class="border-b") {
                                                    td(class="px-4 py-2 bg-gray-50 font-medium text-gray-700 border-r") { "File Size" }
                                                    td(class="px-4 py-2") { (match app.file_size {
                                                        Some(x)=> x.to_string(),
                                                        None=> "N/A".to_string()
                                                    }) }
                                                }
                                                tr {
                                                    td(class="px-4 py-2 bg-gray-50 font-medium text-gray-700 border-r") { "Version" }
                                                    td(class="px-4 py-2") { (app.version) }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Tabs
                            div(class="mt-4 border-b") {
                                div(class="flex space-x-4") {
                                    button(
                                        class=format!("px-4 py-2 font-medium {}", if active_tab.get() == TabType::Description { "text-blue-600 border-b-2 border-blue-600" } else { "text-gray-500 hover:text-gray-700" }),
                                        on:click=move |_| active_tab.set(TabType::Description)
                                    ) {
                                        "Description"
                                    }
                                    button(
                                        class=format!("px-4 py-2 font-medium {}", if active_tab.get() == TabType::Introduction { "text-blue-600 border-b-2 border-blue-600" } else { "text-gray-500 hover:text-gray-700" }),
                                        on:click=move |_| active_tab.set(TabType::Introduction)
                                    ) {
                                        "Introduction"
                                    }
                                    button(
                                        class=format!("px-4 py-2 font-medium {}", if active_tab.get() == TabType::Download { "text-blue-600 border-b-2 border-blue-600" } else { "text-gray-500 hover:text-gray-700" }),
                                        on:click=move |_| active_tab.set(TabType::Download)
                                    ) {
                                        "Download"
                                    }
                                    button(
                                        class=format!("px-4 py-2 font-medium {}", if active_tab.get() == TabType::ServiceNotes { "text-blue-600 border-b-2 border-blue-600" } else { "text-gray-500 hover:text-gray-700" }),
                                        on:click=move |_| active_tab.set(TabType::ServiceNotes)
                                    ) {
                                        "Service Notes"
                                    }
                                }
                            }

                            // Tab content
                            div(class="mt-4 space-y-4 max-h-[70vh] overflow-y-auto") {
                                // Description Tab
                                div(style=if active_tab.get() == TabType::Description { "display: block" } else { "display: none" }) {
                                    div {
                                        p(class="text-gray-600") { (app.description) }
                                    }
                                }

                                // Introduction Tab
                                div(style=if active_tab.get() == TabType::Introduction { "display: block" } else { "display: none" }) {
                                    p(class="text-gray-600") { (app.introduction) }
                                }

                                // Download Tab
                                div(style=if active_tab.get() == TabType::Download { "display: block" } else { "display: none" }) {
                                    div {
                                        p(class="text-sm font-medium text-gray-500") { "Git URL" }
                                        p(class="text-gray-600 break-all") { (app.download.git_url) }

                                        p(class="text-sm font-medium text-gray-500 mt-2") { "Branch" }
                                        p(class="text-gray-600") { (app.download.branch) }
                                    }
                                }

                                // Service Notes Tab
                                div(style=if active_tab.get() == TabType::ServiceNotes { "display: block" } else { "display: none" }) {
                                    p(class="text-gray-600") { (app.service_notes) }
                                }
                            }

                            // Modal footer
                            div(class="mt-6 flex justify-end space-x-3 border-t pt-3") {
                                button(on:click=close_modal, class="px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300") {
                                    "Close"
                                }
                                button(
                                    on:click=move |_| {
                                        let app = app_clone.clone();
                                        handle_install(app);
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
