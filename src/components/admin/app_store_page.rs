use sycamore::futures::spawn_local;
use sycamore::prelude::*;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct AIApp {
    name: String,
    version: String,
    description: String,
    icon: String,
    url: String,
    category: String,
    branch: String,
    status: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GitCloneArgs {
    url: String,
    path: String,
}

#[component]
pub fn AppStorePage() -> View {
    let apps = create_signal(Vec::<AIApp>::new());

    spawn_local({
        let apps = apps.clone();
        async move {
            match invoke("get_app_list", to_value(&()).unwrap()).await {
                result => {
                    if let Ok(raw) = from_value::<String>(result) {
                        let arr: Result<Vec<AIApp>, serde_json::Error> = serde_json::from_str(&raw);
                        match arr {
                            Ok(items) => {
                                apps.set(items);
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

    let handle_install = move |app: AIApp| {
        let args = GitCloneArgs {
            url: app.url,
            path: String::from("/mnt/e/ai/apps/Byaidu/PDFMathTranslate"),
        };
        spawn_local(async move {
            match invoke("git_clone", to_value(&args).unwrap()).await {
                result => {
                    match from_value::<Result<String, String>>(result) {
                        Ok(Ok(response)) => {
                            console_log!("Git clone success: {}", response);
                        }
                        Ok(Err(err)) => {
                            console_log!("Git clone failed: {}", err);
                        }
                        Err(e) => {
                            console_log!("Failed to parse response: {:?}", e);
                        }
                    }
                }
            }
        });
    };

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
                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Create At" }
                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Update At" }
                        th(class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider") { "Status" }
                    }
                }
                tbody(class="divide-y divide-gray-200") {
                    Keyed(
                        list=apps,
                        key=|app| app.name.clone(),
                        view=move |app| {
                            let app_clone = app.clone();
                            view! {
                                tr(class="hover:bg-gray-50 transition-colors duration-200") {
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-900") { (app.name) }
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (app.version.clone()) }
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (app.url.clone()) }
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (app.branch.clone()) }
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (app.created_at.clone()) }
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") { (app.updated_at.clone()) }
                                    td(class="px-6 py-4 whitespace-nowrap text-sm text-gray-500") {
                                        button(
                                            class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition duration-150 ease-in-out",
                                            on:click=move |_| handle_install(app_clone.clone())
                                        ) { "Install" }
                                    }
                                }
                            }
                        }
                    )
                }
            }
        }
    }
}
