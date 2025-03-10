use crate::components::admin::AdminLayout;
use crate::components::banner::Banner;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

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
    view! {
        main(class=" mx-auto") {

            AdminLayout()
        }
    }
}
