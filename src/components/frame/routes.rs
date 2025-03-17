use crate::components::{
    admin::{app_store_page::AppStorePage, dashboard_page::*},
    file_page::FilePage,
    settings_page::SettingsPage,
};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Route, Clone, PartialEq, Debug)]
pub enum AdminRoute {
    #[to("/")]
    Dashboard,
    #[to("/appstore")]
    AppStore,
    #[to("/settings")]
    Settings,
    #[to("/file")]
    File,
    #[not_found]
    NotFound,
}

#[component]
pub fn AdminRouter() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AdminRoute>| {
                view! {
                    (match route.get_clone(){
                        AdminRoute::Dashboard => view! {
                            DashboardPage()
                        },
                        AdminRoute::AppStore => view! {
                            AppStorePage()
                        },
                        AdminRoute::Settings => view! {
                            SettingsPage()
                        },
                        AdminRoute::File => view! {
                            FilePage()
                        },
                        AdminRoute::NotFound => view! { "404 Not Found" },
                    })
                }
            }
        )
    }
}
