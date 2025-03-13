use sycamore::prelude::*;

use crate::components::{
    global_settings_page::GlobalSettingsPage, uv_pythons_page::UVPythonsPage,
    uv_settings_page::UVSettingsPage,
};

#[component]
pub fn SettingsPage() -> View {
    let active_tab = create_signal(0); // 0: UV配置, 1: Python环境
    view! {
        div( ) {
            // 标签页导航
            div(class="flex space-x-4 border-b border-gray-200") {
                button(
                    class=if active_tab.get() == 0 {
                        "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                    } else {
                        "px-4 py-2 text-gray-500 hover:text-gray-700"
                    },
                    on:click=move |_| active_tab.set(0)
                ) { "Global settings" }
                button(
                    class=if active_tab.get() == 1 {
                        "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                    } else {
                        "px-4 py-2 text-gray-500 hover:text-gray-700"
                    },
                    on:click=move |_| active_tab.set(1)
                ) { "UV Config" }
                button(
                    class=if active_tab.get() == 2 {
                        "px-4 py-2 text-blue-600 border-b-2 border-blue-600 font-medium"
                    } else {
                        "px-4 py-2 text-gray-500 hover:text-gray-700"
                    },
                    on:click=move |_| active_tab.set(2)
                ) { "Python Environment" }
            }

            div(class="p-4") {
                // 标签页内容
                (match active_tab.get() {
                    0 => GlobalSettingsPage(),
                    1 => UVSettingsPage(),
                    2 => UVPythonsPage(),
                    _ => view! { "" }
                })
            }
        }
    }
}
