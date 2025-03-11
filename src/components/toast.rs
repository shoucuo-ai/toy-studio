use sycamore::{futures::spawn_local, prelude::*};

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct Toast {
    pub message: String,
    pub toast_type: ToastType,
}

#[component(inline_props)]
pub fn ToastNotification(
    toast: Signal<Option<Toast>>,
    duration_ms: Option<u32>,
) -> View  {
    let visible = create_signal(false);

    create_effect(move || {
        if toast.get_clone().is_some() {
            visible.set(true);
            let duration = duration_ms.unwrap_or(3000);
            let visible = visible.clone();
            spawn_local(async move {
                gloo_timers::future::sleep(std::time::Duration::from_millis(duration as u64)).await;
                visible.set(false);
            });
        }
    });

    let toast_class = move || {
        let base_class = "fixed top-4 right-4 p-4 rounded-lg shadow-lg transform transition-all duration-500 ";
        let visibility_class = if visible.get() {
            "translate-x-0 opacity-100"
        } else {
            "translate-x-full opacity-0"
        };
        let type_class = match &toast.get_clone() {
            Some(t) => match t.toast_type {
                ToastType::Success => "bg-green-500 text-white",
                ToastType::Error => "bg-red-500 text-white",
            },
            None => "",
        };
        format!("{}{} {}", base_class, visibility_class, type_class)
    };

    view! {
        (if toast.get_clone().is_some() {
            view! {
                div(class=toast_class) {
                    (toast.get_clone().as_ref().unwrap().message.clone())
                }
            }
        } else {
            view! {}
        })
    }
}