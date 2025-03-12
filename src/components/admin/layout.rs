use crate::components::admin::routes::{AdminRoute, AdminRouter};
use crate::store::AppConfig;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn AdminLayout() -> View {
    view! {
        div(class="min-h-screen flex flex-col bg-gray-50") {
            // 顶部导航栏
            header(class="bg-white shadow-sm border-b border-gray-200 h-16 flex items-center px-8") {
                h1(class="text-xl font-bold text-gray-800") { "AI Studio" }
            }

            // 下半部分的左右布局
            div(class="flex flex-1") {
                // 左侧边栏
                div(class="w-64 bg-white shadow-lg border-r border-gray-200") {
                    AdminSidebar()
                }
                // 主内容区
                main(class="flex-1 p-8 bg-gray-50") {
                    AdminRouter()
                }
            }
        }
    }
}

#[component]
fn AdminSidebar() -> View {
    let config = use_context::<Signal<AppConfig>>();

    let handle_click = |route: AdminRoute| {
        move |_| {
            let path = match route {
                AdminRoute::Dashboard => "/",
                AdminRoute::AppStore => "/appstore",
                AdminRoute::Settings => "/settings",
                AdminRoute::NotFound => "/404",
                AdminRoute::Todo => "/todo",
            };
            navigate(path);
        }
    };

    view! {
        nav(class="h-full py-6") {
            div(class="px-6 mb-8") {
                h1(class="text-xl font-bold text-gray-800") { "Admin Panel" }
            }
            ul(class="space-y-1") {
                li {
                    a(
                        href="/",
                        class="flex items-center px-6 py-3 text-gray-700 hover:bg-indigo-50 hover:text-indigo-600 rounded-lg mx-2 transition-all duration-200 font-medium",
                        on:click=handle_click(AdminRoute::Dashboard)
                    ) {
                        span(class="mr-3") {
                            i(class="fas fa-home")
                        }
                        "Dashboard"
                    }
                }
                li {
                    a(
                        href="/appstore",
                        class="flex items-center px-6 py-3 text-gray-700 hover:bg-indigo-50 hover:text-indigo-600 rounded-lg mx-2 transition-all duration-200 font-medium",
                        on:click=handle_click(AdminRoute::AppStore)
                    ) {
                        span(class="mr-3") {
                            i(class="fas fa-store")
                        }
                        "App Store"
                    }
                }
                li {
                    a(
                        href="/settings",
                        class="flex items-center px-6 py-3 text-gray-700 hover:bg-indigo-50 hover:text-indigo-600 rounded-lg mx-2 transition-all duration-200 font-medium",
                        on:click=handle_click(AdminRoute::Settings)
                    ) {
                        span(class="mr-3") {
                            i(class="fas fa-cog")
                        }
                        "System Settings"
                    }
                }
                li {
                    a(
                        href="/todo",
                        class="flex items-center px-6 py-3 text-gray-700 hover:bg-indigo-50 hover:text-indigo-600 rounded-lg mx-2 transition-all duration-200 font-medium",
                        on:click=handle_click(AdminRoute::Todo)
                    ) {
                        span(class="mr-3") {
                            i(class="fas fa-cog")
                        }
                        "Todo"
                    }
                }
            }
        }
    }
}
