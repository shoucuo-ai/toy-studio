pub mod app;
pub mod common;
pub mod components;

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
