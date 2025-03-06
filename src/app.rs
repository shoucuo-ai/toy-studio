use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
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
    let name = create_signal(String::new());
    let greet_msg = create_signal(String::new());

    let greet = move |e: SubmitEvent| {
        e.prevent_default();
        spawn_local_scoped(async move {
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let args = serde_wasm_bindgen::to_value(&GreetArgs {
				name: &name.get_clone()
			})
			.unwrap();
            let new_msg = invoke("greet", args).await;
            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    view! {
        main(class="container") {
            h1 {
                "Welcome to Tauri + Sycamore"
            }

            div(class="row") {
                a(href="https://tauri.app", target="_blank") {
                    img(src="public/tauri.svg", class="logo tauri", alt="Tauri logo")
                }
                a(href="https://sycamore.dev", target="_blank") {
                    img(src="public/sycamore.svg", class="logo sycamore", alt="Sycamore logo")
                }
            }
            p {
                "Click on the Tauri and Sycamore logos to learn more."
            }

            form(class="row", on:submit=greet) {
                input(id="greet-input", bind:value=name, placeholder="Enter a name...")
                button(r#type="submit") {
                    "Greet"
                }
            }
            p {
                (greet_msg)
            }
        }
    }
}
