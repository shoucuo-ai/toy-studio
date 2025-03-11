pub mod app;
pub mod components;
pub mod store;

pub use app::App;
use components::titlebar::TitleBar;
use sycamore::prelude::*;

#[component]
pub fn Main() -> View {
    view! {
        TitleBar()
        App()
    }
}
