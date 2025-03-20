use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use crate::common::{invoke_for_string, invoke_tauri, Product};
use crate::components::{AdminLayout, AdminRoute, Toast, ToastNotification, ToastType};

#[component]
pub fn DashboardPage() -> View {
    let apps = create_signal(Vec::<Product>::new());
    let toast = create_signal(None::<Toast>);
    let menu_open = create_signal(false);

    let load_products = async move || {
        let apps = apps.clone();
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
    };
    spawn_local(async move {
        load_products().await;
    });

    let handle_launch_product = move |product_id: String| {
        let toast = toast.clone();
        spawn_local(async move {
            let json = serde_json::json!({
                "file": product_id.clone()
            });
            let args = serde_wasm_bindgen::to_value(&json);

            match args {
                Err(e) => {
                    let message = format!("Failed to parse args: {:?}", e);
                    toast.set(Some(Toast {
                        message: message,
                        toast_type: ToastType::Error,
                    }));
                }
                Ok(args) => {
                    let _msg = invoke_tauri("product_startup", args).await;
                    toast.set(Some(Toast {
                        message: "Product launched successfully".to_string(),
                        toast_type: ToastType::Success,
                    }));
                    load_products().await;
                }
            }
        });
    };

    let handle_open_directory = move |product_id: String| {
        let toast = toast.clone();
        menu_open.clone().set(false);
        spawn_local(async move {
            let json: serde_json::Value = serde_json::json!({
                "file": product_id.clone()
            });
            let args = serde_wasm_bindgen::to_value(&json);

            match args {
                Err(e) => {
                    let message = format!("Failed to parse args: {:?}", e);
                    toast.set(Some(Toast {
                        message: message,
                        toast_type: ToastType::Error,
                    }));
                }
                Ok(args) => {
                    let _x = invoke_for_string("open_product_directory", args).await;
                    toast.set(Some(Toast {
                        message: "Open product directory successfully".to_string(),
                        toast_type: ToastType::Success,
                    }));
                }
            };
        });
    };

    let handle_uninstall = move |product_id: String| {
        let toast = toast.clone();
        spawn_local(async move {
            let json = serde_json::json!({
                "product_id": product_id.clone()
            });
            let args = serde_wasm_bindgen::to_value(&json);

            match args {
                Err(e) => {
                    let message = format!("Failed to parse args: {:?}", e);
                    toast.set(Some(Toast {
                        message: message,
                        toast_type: ToastType::Error,
                    }));
                }
                Ok(args) => match invoke_for_string("uninstall_product", args).await {
                    Some(_) => {
                        toast.set(Some(Toast {
                            message: "Product uninstalled successfully".to_string(),
                            toast_type: ToastType::Success,
                        }));
                    }
                    None => {
                        toast.set(Some(Toast {
                            message: "Failed to uninstall product".to_string(),
                            toast_type: ToastType::Error,
                        }));
                    }
                },
            }
        });
    };

    let handle_reinstall = move |product_id: String| {
        let toast = toast.clone();
        spawn_local(async move {
            let json = serde_json::json!({
                "product_id": product_id.clone()
            });
            let args = serde_wasm_bindgen::to_value(&json);

            match args {
                Err(e) => {
                    let message = format!("Failed to parse args: {:?}", e);
                    toast.set(Some(Toast {
                        message: message,
                        toast_type: ToastType::Error,
                    }));
                }
                Ok(args) => match invoke_for_string("reinstall_product", args).await {
                    Some(_) => {
                        toast.set(Some(Toast {
                            message: "Product reinstalled successfully".to_string(),
                            toast_type: ToastType::Success,
                        }));
                    }
                    None => {
                        toast.set(Some(Toast {
                            message: "Failed to reinstall product".to_string(),
                            toast_type: ToastType::Error,
                        }));
                    }
                },
            }
        });
    };

    let handle_offline_import = move |product_id: String| {
        let toast = toast.clone();
        spawn_local(async move {
            let json = serde_json::json!({
                "product_id": product_id.clone()
            });
            let args = serde_wasm_bindgen::to_value(&json);

            match args {
                Err(e) => {
                    let message = format!("Failed to parse args: {:?}", e);
                    toast.set(Some(Toast {
                        message: message,
                        toast_type: ToastType::Error,
                    }));
                }
                Ok(args) => match invoke_for_string("offline_import_product", args).await {
                    Some(_) => {
                        toast.set(Some(Toast {
                            message: "Product imported successfully".to_string(),
                            toast_type: ToastType::Success,
                        }));
                    }
                    None => {
                        toast.set(Some(Toast {
                            message: "Failed to import product".to_string(),
                            toast_type: ToastType::Error,
                        }));
                    }
                },
            }
        });
    };

    view! {
        AdminLayout(current_route=AdminRoute::Dashboard, inner_view=view! {
            ToastNotification(toast=toast, duration_ms=3000u32)
            div(class="flex space-x-4 border-b border-gray-200") {
                button(class= "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium") {
                    "Dashboard"
                }
            }
            div(class="bg-gray-100 p-4") {
                Keyed(list=apps, view=move |app|{
                    let app_id = app.id.clone();
                    let app_id_for_menu = app.id.clone();
                    view! {
                        div(class="bg-white rounded-lg shadow-md p-4 hover:shadow-lg transition-shadow ") {
                                div(class="flex flex-row justify-between items-center gap-4") {
                                    div(class="flex-grow") {
                                        h3(class="text-lg font-semibold text-gray-800") { (app.name) }
                                        p(class="text-sm text-gray-600 mt-1") { (app.description) }
                                    }
                                    div(class="relative") {


                                        button(
                                            class="text-gray-500 hover:text-gray-700 text-sm",
                                            on:click=move |_| menu_open.set(!menu_open.get())
                                        ) {
                                            img(src="public/ellipsis.svg", class="w-6 h-6")
                                        }

                                        (if menu_open.get_clone() {
                                            view! {
                                                div(
                                                    class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10 py-1"
                                                ) {
                                                    button(
                                                        class="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                        on:click={
                                                            let id = app_id_for_menu.clone();
                                                            move |_| handle_open_directory(id.clone())
                                                        }
                                                    ) { "Open Directory" }

                                                    button(
                                                        class="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                        on:click={
                                                            let id = app_id_for_menu.clone();
                                                            move |_| handle_uninstall(id.clone())
                                                        }
                                                    ) { "Uninstall" }

                                                    button(
                                                        class="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                        on:click={
                                                            let id = app_id_for_menu.clone();
                                                            move |_| handle_reinstall(id.clone())
                                                        }
                                                    ) { "Reinstall" }

                                                    button(
                                                        class="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                                                        on:click={
                                                            let id = app_id_for_menu.clone();
                                                            move |_| handle_offline_import(id.clone())
                                                        }
                                                    ) { "Offline Import" }
                                                }

                                                // 点击其他区域关闭菜单
                                                div(
                                                    class="fixed inset-0 h-full w-full z-0",
                                                    on:click=move |_| menu_open.set(false)
                                                ) {}
                                            }
                                        } else {
                                            view! {}
                                        })
                                    }
                                    (if let Some(true) = app.running {
                                        view! {
                                            "Running"
                                        }
                                    } else {
                                        view! {
                                            "Not Running"
                                        }
                                    })
                                    button(
                                        class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-md text-sm transition-colors",
                                        on:click=move |_| {
                                            let product_id = app_id.clone();
                                            handle_launch_product(product_id);
                                        }
                                    ) {
                                        "Launch"
                                    }
                                }
                            }
                        }
                    },
                    key=|app| app.id.clone(),
                )
            }
        })
    }
}
