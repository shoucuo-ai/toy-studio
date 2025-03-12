use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};

use crate::components::global_settings_page::GlobalSettingsPage;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

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
            match invoke("get_uv_cache_dir", to_value(&()).unwrap()).await {
                result => {
                    if let Ok(dir) = from_value::<String>(result) {
                        cache_dir.set(dir);
                    }
                }
            }
        }
    });

    let handle_browse = {
        let cache_dir = cache_dir.clone();
        move |_| {
            let cache_dir = cache_dir.clone();
            spawn_local(async move {
                let value = invoke("select_directory", JsValue::NULL).await;
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
        div {
            h2(class="text-2xl font-bold mb-6") { "UV Config" }
            form(class="space-y-6 max-w-2xl") {
                div(class="space-y-2") {
                    label(class="block text-sm font-medium text-gray-700") { "Cache Directory" }
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
                    p(class="mt-1 text-sm text-gray-500") { "Please specify the cache directory location for the UV package manager" }
                }
                button(
                    class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-150 ease-in-out",
                    r#type="submit"
                ) { "Save Configuration" }
            }
        }
    }
}
