use sycamore::prelude::*;

use crate::components::{AdminRouter, AdminSidebar};

#[component]
pub fn AdminLayout() -> View {
    view! {
        main(class="h-screen flex flex-col bg-gray-50") {
            // 顶部导航栏
            header(class="bg-white shadow-sm border-b border-gray-200 h-16 flex items-center px-8") {
                h1(class="text-xl font-bold text-gray-800") { "AI Studio" }
            }

            // 下半部分的左右布局
            div(class="flex flex-1 overflow-hidden") {
                // 左侧边栏
                div(class="w-16 bg-white shadow-md border-r border-gray-100 transition-all duration-300") {
                    AdminSidebar()
                }
                // 主内容区
                main(class="flex-1 bg-gray-50 overflow-auto") {
                    AdminRouter()
                }
            }
        }
    }
}
