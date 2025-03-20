use crate::{common::AppConfig, components::AdminRouter};
use sycamore::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn App() -> View {
    let config = create_signal(AppConfig::default());

    // 加载配置
    spawn_local({
        let config = config.clone();
        async move {
            let data = AppConfig::load().await;
            match data {
                Ok(loaded_config) => {
                    config.set(loaded_config);
                }
                Err(e) => {
                    console_log!("load config error: {:?}", e);
                }
            }
        }
    });

    provide_context(config);

    view! {
        AdminRouter()
    }
}
