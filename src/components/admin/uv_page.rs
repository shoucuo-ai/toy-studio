use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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
pub fn UVPage() -> View {
    let active_tab = create_signal(0); // 0: UV配置, 1: Python环境
    let cache_dir = create_signal(String::new());
    let python_envs = create_signal(Vec::<PythonEnv>::new());

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

    // 获取Python环境列表
    spawn_local({
        let python_envs = python_envs.clone();
        async move {
            match invoke("get_python_envs", to_value(&()).unwrap()).await {
                result => {
                    if let Ok(envs_json) = from_value::<String>(result) {
                        let envs: Result<Vec<PythonEnv>, serde_json::Error> =
                            serde_json::from_str(&envs_json);
                        match envs {
                            Ok(envs) => {
                                python_envs.set(envs);
                            }
                            Err(e) => {
                                    console_log!("error: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    });

    create_effect(move || {
        console_log!("cache_dir: {:?}", cache_dir.get_clone());
        console_log!("python_envs: {:?}", python_envs.get_clone());
    });

    view! {
        div(class="container mx-auto px-4 py-8") {
            // 标签页导航
            div(class="flex space-x-4 mb-6 border-b border-gray-200") {
                button(
                    class=if active_tab.get() == 0 {
                        "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                    } else {
                        "px-4 py-2 text-gray-500 hover:text-gray-700"
                    },
                    on:click=move |_| active_tab.set(0)
                ) { "UV Config" }
                button(
                    class=if active_tab.get() == 1 {
                        "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                    } else {
                        "px-4 py-2 text-gray-500 hover:text-gray-700"
                    },
                    on:click=move |_| active_tab.set(1)
                ) { "Python Environment" }
            }

            // 标签页内容
            div(class="mt-6") {
                (match active_tab.get() {
                    0 => view! {
                        div {
                            h2(class="text-2xl font-bold mb-6") { "UV Config" }
                            form(class="space-y-6 max-w-2xl") {
                                div(class="space-y-2") {
                                    label(class="block text-sm font-medium text-gray-700") { "Cache Directory" }
                                    div(class="relative rounded-md shadow-sm") {
                                        input(
                                            class="block w-full rounded-md border-gray-300 pl-4 pr-12 py-3 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 sm:text-sm transition duration-150 ease-in-out hover:border-blue-400",
                                            r#type="text",
                                            placeholder="Enter cache directory path...",
                                            bind:value=cache_dir
                                        )
                                        span(class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none") {
                                            // You can add an icon here
                                            i(class="text-gray-400") { "📁" }
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
                    },
                    1 => view! {
                        div(class="overflow-x-auto") {
                            h2(class="text-2xl font-bold mb-6") { "Python Environments" }
                            table(class="w-full border-collapse bg-white shadow-sm rounded-lg overflow-hidden") {
                                thead {
                                    tr(class="bg-gray-50 border-b border-gray-200") {
                                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Name" }
                                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Version" }
                                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Path" }
                                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "OS" }
                                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Architecture" }
                                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Implementation" }
                                    }
                                }
                                tbody(class="divide-y divide-gray-200") {
                                    Keyed(
                                        list=python_envs,
                                        key=|env| env.key.clone(),
                                        view=|env| view! {
                                            tr(class="hover:bg-gray-50 transition-colors duration-200") {
                                                td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-900") { (env.key) }
                                                td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (env.version.clone().unwrap_or_default()) }
                                                td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (env.path.clone().unwrap_or_default()) }
                                                td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (env.os.clone().unwrap_or_default()) }
                                                td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (env.arch.clone().unwrap_or_default()) }
                                                td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (env.implementation.clone().unwrap_or_default()) }
                                            }
                                        }
                                    )
                                }
                            }
                        }
                    },
                    _ => view! { "" }
                })
            }
        }
    }
}
