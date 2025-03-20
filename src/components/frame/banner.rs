use sycamore::prelude::*;

#[component]
pub fn Banner() -> View {
    view! {
        div(class="flex justify-between border-b border-gray-200 items-center py-2 bg-white shadow-md") {
            // 左侧 Logo 区域
            div(class="flex items-center") {
                div(class="flex items-center") {
                    div(class="w-16 ") {
                        img(src="/public/tauri.svg", alt="Logo", class="h-8 w-8 mx-auto")
                    }
                    span(class="text-lg font-semibold") { "Toy Studio" }
                }
            }

            // 中间搜索框
            // div(class="flex-1 mx-8 w-32") {
            //     div(class="relative") {
            //         input(
            //             placeholder="search...",
            //             class="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
            //         )
            //         span(class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400") { "🔍" }
            //     }
            // }

            // 右侧通知和用户信息
            // div(class="flex items-center gap-2") {
            //     div(class="relative w-8") {
            //         span(class="text-xl cursor-pointer") {
            //             img(src="/public/message.svg", alt="User Avatar", class="h-6 w-6 rounded-full")
            //         }
            //         span(class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-6 h-4 flex items-center justify-center") { "3" }
            //     }

            //     div(class="relative w-8") {
            //         span(class="text-xl cursor-pointer") {
            //             img(src="/public/alert.svg", alt="User Avatar", class="h-6 w-6 rounded-full")
            //         }
            //         span(class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-6 h-4 flex items-center justify-center") { "3" }
            //     }

            //     div(class="flex items-center gap-2 pr-4") {
            //         img(src="/public/user.svg", alt="User Avatar", class="h-6 w-6 rounded-full")
            //         span(class="text-sm font-medium") { "admin" }
            //     }
            // }
        }
    }
}
