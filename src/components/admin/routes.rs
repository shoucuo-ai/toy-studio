use crate::components::admin::{app_store_page::AppStorePage, dashbaord_page::*, settings_page::SettingsPage, uv_page::UVPage};
use sycamore::prelude::*;
use sycamore_router::{ HistoryIntegration, Route, Router};

#[derive(Route, Clone, PartialEq)]
pub enum AdminRoute {
    #[to("/")]
    Dashboard,
    #[to("/uv")]
    UV,
    #[to("/appstore")]
    AppStore,
    #[to("/settings")]
    Settings,
    #[not_found]
    NotFound,
}

#[component]
pub fn AdminRouter() -> View {
    let router_view = move |route: ReadSignal<AdminRoute>| {
        let route_value = route.get_clone();
        match route_value {
            AdminRoute::Dashboard => view! { DashboardPage() },
            AdminRoute::UV => view! { UVPage() },
            AdminRoute::AppStore => view! { AppStorePage() },
            AdminRoute::Settings => view! { SettingsPage() },
            AdminRoute::NotFound => view! { "404 Not Found" },
        }
    };

    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=router_view
        )
    }
}
