use crate::components::admin::routes::{AdminRoute, AdminRouter};
use crate::components::admin::ADMIN_STYLES;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn AdminLayout() -> View {
    view! {
        style { (ADMIN_STYLES) }
        div(class="admin-layout") {
            div(class="admin-sidebar") {
                AdminSidebar()
            }
            div(class="admin-content") {
                AdminRouter()
            }
        }
    }
}

#[component]
fn AdminSidebar() -> View {
    let handle_click = |route: AdminRoute| {
        move |_| {
            let path = match route {
                AdminRoute::Dashboard => "/",
                AdminRoute::UV => "/uv",
                AdminRoute::AppStore => "/appstore",
                AdminRoute::Settings => "/settings",
                AdminRoute::NotFound => "/404",
            };
            navigate(path);
        }
    };

    view! {
        nav(class="sidebar") {
            ul(class="sidebar-menu") {
                li {
                    a(
                        href="/",
                        on:click=handle_click(AdminRoute::Dashboard)
                    ) { "Dashboard" }
                }
                li {
                    a(
                        href="/uv",
                        on:click=handle_click(AdminRoute::UV)
                    ) { "UV" }
                }
                li {
                    a(
                        href="/appstore",
                        on:click=handle_click(AdminRoute::AppStore)
                    ) { "App Store" }
                }
                li {
                    a(
                        href="/settings",
                        on:click=handle_click(AdminRoute::Settings)
                    ) { "System Settings" }
                }
            }
        }
    }
}

#[component]
fn AdminContent() -> View {
    view! {
        main(class="content") {
            h2 { "Welcome to Admin Dashboard" }
            p { "Please select an option from the menu on the left." }
        }
    }
}
