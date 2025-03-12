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
pub fn UVPythonsPage() -> View {
    let python_envs = create_signal(Vec::<PythonEnv>::new());

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
        console_log!("python_envs: {:?}", python_envs.get_clone());
    });

    view! {
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
    }
}
