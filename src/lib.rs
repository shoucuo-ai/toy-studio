pub mod app;
pub mod components;
pub mod store;
pub mod common;

pub use app::App;
use components::TitleBar;
use sycamore::prelude::*;

#[component]
pub fn Main() -> View {
    view! {
        TitleBar()
        App()
    }
}
