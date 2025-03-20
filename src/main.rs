#![allow(non_snake_case)]

use toy_studio_ui::*;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
