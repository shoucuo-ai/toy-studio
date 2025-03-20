use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};

use crate::common::{invoke_for_string, invoke_tauri};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct PythonVersionParts {
    major: i32,
    minor: i32,
    patch: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct PythonEnv {
    key: String,
    version: Option<String>,
    version_parts: PythonVersionParts,
    path: Option<String>,
    symlink: Option<String>,
    url: Option<String>,
    os: Option<String>,
    variant: Option<String>,
    implementation: Option<String>,
    arch: Option<String>,
    libc: Option<String>,
}

#[component]
pub fn UVSettingsPage() -> View {
    let cache_dir = create_signal(String::new());

    // 获取UV缓存目录
    spawn_local({
        let cache_dir = cache_dir.clone();
        async move {
            let result = invoke_for_string("uv_get_cache_dir", JsValue::NULL).await;
            console_log!("result: {:?}", result);
            match result {
                Some(dir) => {
                    cache_dir.set(dir);
                }
                None => {
                    console_log!("Failed to get UV cache directory");
                }
            }
        }
    });

    let handle_browse = {
        let cache_dir = cache_dir.clone();
        move |_| {
            let cache_dir = cache_dir.clone();
            spawn_local(async move {
                let value = invoke_tauri("select_directory", JsValue::NULL).await;
                if let Some(result) = value.as_string() {
                    if !result.is_empty() {
                        cache_dir.set(result);
                    }
                }
            });
        }
    };

    let handle_input = move |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        cache_dir.set(target.value());
    };

    create_effect(move || {
        console_log!("cache_dir: {:?}", cache_dir.get_clone());
    });
    view! {
        div(class="bg-gray-50") {
            form(class="space-y-6 bg-white shadow rounded-lg p-6") {
                div(class="space-y-4") {
                    div(class="flex flex-col") {
                        label(class="block text-sm font-medium text-gray-700 mb-1") {
                            "Cache Directory"
                        }
                        div(class="flex") {
                            input(
                                class="flex-1 appearance-none block w-full px-3 py-2 border border-gray-300 rounded-l-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                                r#type="text",
                                placeholder="Select cache directory",
                                value=cache_dir.get_clone(),
                                on:input=handle_input
                            )
                            button(
                                class="inline-flex items-center px-4 py-2 border border-l-0 border-gray-300 rounded-r-md shadow-sm bg-gray-50 text-sm font-medium text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-1 focus:ring-indigo-500 focus:border-indigo-500",
                                r#type="button",
                                on:click=handle_browse
                            ) {
                                "Browse..."
                            }
                        }
                        p(class="mt-1 text-sm text-gray-500") {
                            "Please specify the cache directory location for the UV package manager"
                        }
                    }
                }
                div(class="flex justify-end space-x-3 mt-6") {
                    button(
                        class="inline-flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                        r#type="submit"
                    ) {
                        "Save Configuration"
                    }
                }
            }
        }
    }
}
