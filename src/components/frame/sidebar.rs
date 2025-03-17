use crate::components::AdminRoute;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[component(inline_props)]
pub fn SidebarLi(text: String, img: String, activate: bool, route: AdminRoute) -> View {
    let handle_click = move |_| {
        console_log!("route: SidebarLi:{:?}", &route);
        let path = match route {
            AdminRoute::Dashboard => "/",
            AdminRoute::AppStore => "/appstore",
            AdminRoute::Settings => "/settings",
            AdminRoute::File => "/file",
            AdminRoute::NotFound => "/404",
        };
        navigate(path);
    };

    view! {
        div(class="w-full mb-0") {
            a(
                class=if activate {
                    "cursor-pointer group flex flex-col items-center p-2 rounded-lg transition-all duration-200 bg-blue-100".to_string()
                } else {
                    "cursor-pointer group flex flex-col items-center p-2 rounded-lg transition-all duration-200 hover:bg-blue-50 ".to_string()
                },
                on:click=handle_click
            ) {
                div(class="w-10 h-10 flex items-center justify-center rounded-lg bg-blue-100 group-hover:bg-blue-200 transition-colors") {
                    img(src=img, class="w-6 h-6")
                }
                span(class="mt-2 text-sm font-medium text-gray-700 group-hover:text-blue-600") {(text) }
            }
        }
    }
}

#[component(inline_props)]
pub fn AdminSidebar(current_route: AdminRoute) -> View {
    // 创建current_route的克隆，以便在多个地方使用
    let current_route_clone1 = current_route.clone();
    let current_route_clone2 = current_route.clone();
    let current_route_clone3 = current_route.clone();

    view! {
        nav(class="flex flex-col min-h-screen justify-between bg-white shadow-lg border-r pt-1 border-gray-200") {
            // Top navigation items
            div(class="flex-1 overflow-y-auto p-px") {
                div(class="flex flex-col items-center space-y-6 gap-1") {
                    SidebarLi(
                        text="Index".to_string(),
                        img="/public/home.svg".to_string(),
                        activate=current_route==AdminRoute::Dashboard,
                        route=AdminRoute::Dashboard
                    )

                    SidebarLi(
                        text="App".to_string(),
                        img="/public/store.svg".to_string(),
                        activate=current_route_clone1==AdminRoute::AppStore,
                        route=AdminRoute::AppStore
                    )

                    SidebarLi(
                        text="Files".to_string(),
                        img="/public/file.svg".to_string(),
                        activate=current_route_clone2==AdminRoute::File,
                        route=AdminRoute::File
                    )
                }
            }

            // Bottom settings button
            div(class="sticky bottom-0 w-full border-t border-gray-200 bg-white p-px") {
                SidebarLi(
                    text="Setting".to_string(),
                    img="/public/setting.svg".to_string(),
                    activate=current_route_clone3==AdminRoute::Settings,
                    route=AdminRoute::Settings
                )
            }
        }
    }
}
