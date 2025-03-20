use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::common::invoke_for_data;

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
            let result =
                invoke_for_data::<Vec<PythonEnv>>("uv_get_python_envs", JsValue::NULL).await;

            match result {
                Ok(envs) => {
                    python_envs.set(envs);
                }
                Err(e) => {
                    console_log!("error: {:?}", e);
                }
            }
        }
    });

    view! {
        div(class="bg-white shadow rounded-lg p-6") {
                    div(class="overflow-x-auto") {
                        table(class="w-full border-collapse") {
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
}
