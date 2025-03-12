use crate::components::toast::{Toast, ToastNotification, ToastType};
use crate::store::AppConfig;
use serde_wasm_bindgen::from_value;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, SubmitEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn GlobalSettingsPage() -> View {
    let toast = create_signal(None::<Toast>);
    let config = use_context::<Signal<AppConfig>>();

    // 保存配置
    let handle_submit = {
        let toast = toast.clone();
        let config = config.clone();
        move |ev: SubmitEvent| {
            ev.prevent_default();
            let config_value = config.get_clone();
            let toast = toast.clone();
            let config = config.clone();
            spawn_local(async move {
                match config_value.save().await {
                    Ok(_) => {
                        toast.set(Some(Toast {
                            message: "Settings saved successfully".to_string(),
                            toast_type: ToastType::Success,
                        }));
                        // 更新全局配置
                        if let Ok(loaded_config) = AppConfig::load().await {
                            config.set(loaded_config);
                        }
                    }
                    Err(e) => {
                        toast.set(Some(Toast {
                            message: format!("Failed to save settings: {}", e),
                            toast_type: ToastType::Error,
                        }));
                    }
                }
            });
        }
    };

    // 更新配置
    let update_language = {
        let config = config.clone();
        move |ev: Event| {
            let target: HtmlSelectElement = ev.target().unwrap().dyn_into().unwrap();
            let mut new_config = config.get_clone();
            new_config.language = target.value();
            config.set(new_config);
        }
    };

    let update_project_root = {
        let config = config.clone();
        move |ev: Event| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            let mut new_config = config.get_clone();
            new_config.project_root_dir = target.value();
            config.set(new_config);
        }
    };

    let update_external_uv = {
        let config = config.clone();
        move |ev: Event| {
            let target: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
            let mut new_config = config.get_clone();
            new_config.enable_external_uv = target.checked();
            config.set(new_config);
        }
    };

    let browse_directory = {
        let config = config.clone();
        let toast = toast.clone();
        move |_| {
            let config = config.clone();
            let toast = toast.clone();
            spawn_local(async move {
                let result = invoke("select_directory", JsValue::NULL).await;
                console_log!("result: {:?}", result);
                match from_value::<String>(result) {
                    Ok(path) => {
                        if path.len() > 0 {
                            let mut new_config = config.get_clone();
                            new_config.project_root_dir = path;
                            config.set(new_config);
                        }
                    }
                    Err(e) => {
                        toast.set(Some(Toast {
                            message: format!("Failed to select directory: {}", e),
                            toast_type: ToastType::Error,
                        }));
                    }
                }
            });
        }
    };

    view! {
        div(class="min-h-screen bg-gray-50 py-8 px-4 sm:px-6 lg:px-8") {
            ToastNotification(toast=toast, duration_ms=3000u32)
            div(class="max-w-2xl mx-auto") {
                h1(class="text-3xl font-bold text-gray-900 mb-8 text-center") {
                    "System Settings"
                }
                form(class="space-y-6 bg-white shadow rounded-lg p-6", on:submit=handle_submit) {
                    div(class="space-y-4") {
                        // 语言设置
                        div(class="flex flex-col") {
                            label(class="block text-sm font-medium text-gray-700 mb-1") {
                                "Language"
                            }
                            select(
                                class="mt-1 block w-full pl-3 pr-10 py-2 text-base border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 rounded-md",
                                value=create_memo(move || config.get_clone().language.clone()),
                                on:change=update_language
                            ) {
                                option(value="zh") { "Chinese" }
                                option(value="en") { "English" }
                            }
                        }

                        // 项目根目录
                        div(class="flex flex-col") {
                            label(class="block text-sm font-medium text-gray-700 mb-1") {
                                "Project Root Directory"
                            }
                            div(class="flex") {
                                input(
                                    class="flex-1 appearance-none block w-full px-3 py-2 border border-gray-300 rounded-l-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                                    r#type="text",
                                    placeholder="Select project root directory",
                                    value=create_memo(move || config.get_clone().project_root_dir.clone()),
                                    on:input=update_project_root
                                )
                                button(
                                    class="inline-flex items-center px-4 py-2 border border-l-0 border-gray-300 rounded-r-md shadow-sm bg-gray-50 text-sm font-medium text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500",
                                    r#type="button",
                                    on:click=browse_directory
                                ) {
                                    "Browse..."
                                }
                            }
                        }

                        // 外部UV设置
                        div(class="flex items-center") {
                            input(
                                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded",
                                r#type="checkbox",
                                id="external-uv",
                                checked=create_memo(move || config.get_clone().enable_external_uv),
                                on:change=update_external_uv
                            )
                            label(
                                class="ml-2 block text-sm text-gray-700",
                                r#for="external-uv"
                            ) {
                                "Enable External UV"
                            }
                        }
                    }

                    // 保存按钮
                    div(class="flex justify-end space-x-3 mt-6") {
                        button(
                            class="inline-flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                            r#type="submit"
                        ) {
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}
