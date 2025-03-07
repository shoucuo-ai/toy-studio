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
        div(class="uv-page") {
            // 标签页导航
            div(class="tab-nav") {
                button(
                    class=if active_tab.get() == 0 { "tab-btn active" } else { "tab-btn" },
                    on:click=move |_| active_tab.set(0)
                ) { "UV配置" }
                button(
                    class=if active_tab.get() == 1 { "tab-btn active" } else { "tab-btn" },
                    on:click=move |_| active_tab.set(1)
                ) { "Python环境" }
            }

            // 标签页内容
            div(class="tab-content") {
                (match active_tab.get() {
                    0 => view! {
                        div(class="uv-config") {
                            h2 { "UV配置" }
                            form {
                                div(class="form-group") {
                                    label { "cache dir" }
                                    input(
                                        r#type="text",
                                        placeholder="cache dir",
                                        bind:value=cache_dir
                                    )
                                }
                                button(r#type="submit") { "save" }
                            }
                        }
                    },
                    1 => view! {
                        div(class="python-env") {
                            h2 { "Python envs" }
                            div(class="env-list") {
                                Keyed(
                                    list=python_envs,
                                    key=|env| env.key.clone(),
                                    view=|env| view! {
                                        div(class="env-item") {
                                            h3 { (env.key) }
                                        }
                                    }
                                )
                            }
                        }
                    },
                    _ => view! { "" }
                })
            }
        }
    }
}
