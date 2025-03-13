use sycamore::prelude::*;
use sycamore_router::navigate;

use crate::components::AdminRoute;

#[component]
pub fn AdminSidebar() -> View {
    let handle_click = |route: AdminRoute| {
        move |_| {
            let path = match route {
                AdminRoute::Dashboard => "/",
                AdminRoute::AppStore => "/appstore",
                AdminRoute::Settings => "/settings",
                AdminRoute::NotFound => "/404",
                AdminRoute::File => "/file",
            };
            navigate(path);
        }
    };

    view! {
        nav(class="flex flex-col min-h-screen justify-between bg-white shadow-lg border-r border-gray-200") {
            // Top navigation items
            div(class="flex-1 overflow-y-auto") {
                ul(class="flex flex-col items-center pt-1 space-y-6") {
                    li(class="w-full mb-0") {
                        a(
                            href="/",
                            class="group flex flex-col items-center p-2 rounded-lg hover:bg-blue-50 transition-all duration-200",
                            on:click=handle_click(AdminRoute::Dashboard)
                        ) {
                            div(class="w-10 h-10 flex items-center justify-center rounded-lg bg-blue-100 group-hover:bg-blue-200 transition-colors") {
                                img(src="/public/home.svg", class="w-6 h-6")

                            }
                            span(class="mt-2 text-sm font-medium text-gray-700 group-hover:text-blue-600") { "Index" }
                        }
                    }
                    li(class="w-full mb-0") {
                        a(
                            href="/appstore",
                            class="group flex flex-col items-center p-2 rounded-lg hover:bg-blue-50 transition-all duration-200",
                            on:click=handle_click(AdminRoute::AppStore)
                        ) {
                            div(class="w-10 h-10 flex items-center justify-center rounded-lg bg-blue-100 group-hover:bg-blue-200 transition-colors") {
                                img(src="/public/store.svg", class="w-6 h-6")
                            }
                            span(class="mt-2 text-sm font-medium text-gray-700 group-hover:text-blue-600") { "App" }
                        }
                    }
                    li(class="w-full mb-0") {
                        a(
                            href="/file",
                            class="group flex flex-col items-center p-2 rounded-lg hover:bg-blue-50 transition-all duration-200",
                            on:click=handle_click(AdminRoute::File)
                        ) {
                            div(class="w-10 h-10 flex items-center justify-center rounded-lg bg-blue-100 group-hover:bg-blue-200 transition-colors") {
                                img(src="/public/file.svg", class="w-6 h-6")
                            }
                            span(class="mt-2 text-sm font-medium text-gray-700 group-hover:text-blue-600") { "Files" }
                        }
                    }
                }
            }

            // Bottom settings button
            div(class="sticky bottom-0 w-full border-t border-gray-200 bg-white pt-1") {
                a(
                    href="/settings",
                    class="group flex flex-col items-center p-2 hover:bg-blue-50 transition-all duration-200",
                    on:click=handle_click(AdminRoute::Settings)
                ) {
                    div(class="w-10 h-10 flex items-center justify-center rounded-lg bg-blue-100 group-hover:bg-blue-200 transition-colors") {
                        img(src="/public/user.svg", class="w-6 h-6")
                    }
                    span(class="mt-2 text-sm font-medium text-gray-700 group-hover:text-blue-600") { "User" }
                }
                a(
                    href="/settings",
                    class="group flex flex-col items-center p-2 hover:bg-blue-50 transition-all duration-200",
                    on:click=handle_click(AdminRoute::Settings)
                ) {
                    div(class="w-10 h-10 flex items-center justify-center rounded-lg bg-blue-100 group-hover:bg-blue-200 transition-colors") {
                        img(src="/public/setting.svg", class="w-6 h-6")
                    }
                    span(class="mt-2 text-sm font-medium text-gray-700 group-hover:text-blue-600") { "Setting" }
                }
            }
        }
    }
}
