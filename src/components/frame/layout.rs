use sycamore::prelude::*;
use crate::components::{AdminRoute, AdminSidebar, Banner};

#[component(inline_props)]
pub fn AdminLayout(current_route: AdminRoute, inner_view: View) -> View {
    view! {
        main(class="h-screen flex flex-col bg-gray-50") {
            // 顶部导航栏
            Banner()

            // 下半部分的左右布局
            div(class="flex flex-1 overflow-hidden") {
                // 左侧边栏
                div(class="w-16 bg-white shadow-md border-r border-gray-100 transition-all duration-300") {
                    AdminSidebar(current_route=current_route)
                }
                // 主内容区
                main(class="flex-1 bg-gray-50 overflow-auto") {
                    (inner_view)
                }
            }
        }
    }
}
