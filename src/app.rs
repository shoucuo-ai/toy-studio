use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{components::layout::AdminLayout, store::AppConfig};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> View {
    let config = create_signal(AppConfig::default());

    // 加载配置
    spawn_local({
        let config = config.clone();
        async move {
            if let Ok(loaded_config) = AppConfig::load().await {
                config.set(loaded_config);
            }
        }
    });

    provide_context(config);
    view! {
        main(class=" mx-auto") {
            AdminLayout()
        }
    }
}
