use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::web::wasm_bindgen::prelude::*;
use uuid::Uuid;
use web_sys::{HtmlInputElement, KeyboardEvent};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Todo {
    title: String,
    completed: bool,
    id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Filter {
    fn url(self) -> &'static str {
        match self {
            Filter::All => "#",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }

    fn get_filter_from_hash() -> Self {
        let hash = window().location().hash().unwrap();

        match hash.as_str() {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppState {
    pub todos: Signal<Vec<Signal<Todo>>>,
    pub filter: Signal<Filter>,
}

impl AppState {
    fn add_todo(&self, title: String) {
        let new = create_signal(Todo {
            title,
            completed: false,
            id: Uuid::new_v4(),
        });
        self.todos.update(|todos| todos.push(new));
    }

    fn remove_todo(&self, id: Uuid) {
        self.todos
            .update(|todos| todos.retain(|todo| todo.with(|todo| todo.id) != id));
    }

    fn todos_left(&self) -> usize {
        self.todos.with(|todos| {
            todos.iter().fold(0, |acc, todo| {
                if todo.with(|todo| todo.completed) {
                    acc
                } else {
                    acc + 1
                }
            })
        })
    }

    fn toggle_complete_all(&self) {
        if self.todos_left() == 0 {
            // make all todos active
            for todo in self.todos.get_clone() {
                if todo.with(|todo| todo.completed) {
                    todo.set(Todo {
                        completed: false,
                        ..todo.get_clone()
                    })
                }
            }
        } else {
            // make all todos completed
            for todo in self.todos.get_clone() {
                if !todo.with(|todo| todo.completed) {
                    todo.set(Todo {
                        completed: true,
                        ..todo.get_clone()
                    })
                }
            }
        }
    }

    fn clear_completed(&self) {
        self.todos
            .update(|todos| todos.retain(|todo| !todo.with(|todo| todo.completed)));
    }
}

const KEY: &str = "todos-sycamore";


#[component]
pub fn TodoPage() -> View {
    // Initialize application state from localStorage.
    let local_storage = window()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");

    let todos = if let Ok(Some(app_state)) = local_storage.get_item(KEY) {
        serde_json::from_str(&app_state).unwrap_or_default()
    } else {
        Default::default()
    };
    let app_state = AppState {
        todos,
        filter: create_signal(Filter::get_filter_from_hash()),
    };
    provide_context(app_state);

    // Set up an effect that runs whenever app_state.todos changes to save the todos to
    // localStorage.
    create_effect(move || {
        app_state.todos.with(|todos| {
            for todo in todos {
                todo.track();
            }
            local_storage
                .set_item(KEY, &serde_json::to_string(todos).unwrap())
                .unwrap();
        });
    });

    let todos_empty = create_selector(move || app_state.todos.with(Vec::is_empty));

    view! {
        div(class="min-h-screen bg-gray-100 py-8 px-4") {
            section(class="max-w-2xl mx-auto bg-white rounded-lg shadow-lg") {
                Header {}
                ((!todos_empty.get()).then(|| view! {
                    List {}
                    Footer {}
                }))
            }
            Copyright {}
        }
    }
}

#[component]
pub fn Copyright() -> View {
    view! {
        footer(class="text-center mt-8 text-gray-600 text-sm") {
            p { "Double click to edit a todo" }
        }
    }
}

#[component]
pub fn Header() -> View {
    let app_state = use_context::<AppState>();
    let input_value = create_signal(String::new());

    let handle_keyup = move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            let task: String = input_value.with(|task| task.trim().to_string());

            if !task.is_empty() {
                app_state.add_todo(task);
                // Reset input field.
                input_value.set("".to_string());
            }
        }
    };

    view! {
        header(class="p-6 border-b border-gray-200") {
            h1(class="text-4xl font-bold text-center text-gray-800 mb-6") { "todos" }
            input(
                class="w-full px-4 py-3 rounded-lg border border-gray-300 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500",
                placeholder="What needs to be done?",
                bind:value=input_value,
                on:keyup=handle_keyup,
            )
        }
    }
}

#[component(inline_props)]
pub fn Item(todo: Signal<Todo>) -> View {
    let app_state = use_context::<AppState>();

    let title = move || todo.with(|todo| todo.title.clone());
    let completed = create_selector(move || todo.with(|todo| todo.completed));
    let id = todo.with(|todo| todo.id);

    let is_editing = create_signal(false);
    let input_ref = create_node_ref();
    let input_value = create_signal("".to_string());

    let toggle_completed = move |_| todo.update(|todo| todo.completed = !todo.completed);

    let handle_dblclick = move |_| {
        is_editing.set(true);
        input_ref
            .get()
            .unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap();
        input_value.set(title());
    };

    let handle_blur = move || {
        is_editing.set(false);

        let value = input_value.with(|value| value.trim().to_string());

        if value.is_empty() {
            app_state.remove_todo(id);
        } else {
            todo.update(|todo| todo.title = value)
        }
    };

    let handle_keyup = move |event: KeyboardEvent| match event.key().as_str() {
        "Enter" => handle_blur(),
        "Escape" => is_editing.set(false),
        _ => {}
    };

    let handle_destroy = move |_| {
        app_state.remove_todo(id);
    };

    // We need a separate signal for checked because clicking the checkbox will detach the binding
    // between the attribute and the view.
    let checked = create_signal(false);
    create_effect(move || {
        // Calling checked.set will also update the `checked` property on the input element.
        checked.set(completed.get())
    });

    let class = move || {
        format!(
            "group flex items-center px-4 py-3 border-b border-gray-200 hover:bg-gray-50 {} {}",
            if completed.get() { "bg-gray-50" } else { "" },
            if is_editing.get() { "editing" } else { "" }
        )
    };

    view! {
        li(class=class()) {
            div(class="flex items-center flex-1") {
                input(
                    class="h-5 w-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500",
                    r#type="checkbox",
                    on:input=toggle_completed,
                    bind:checked=checked
                )
                label(
                    class="ml-3 flex-1 text-gray-700 cursor-pointer",
                    on:dblclick=handle_dblclick
                ) {
                    (title)
                }
                button(
                    class="invisible group-hover:visible text-gray-400 hover:text-red-500",
                    on:click=handle_destroy
                ) {
                    "×"
                }
            }

            (is_editing.get().then(|| view! {
                input(
                    r#ref=input_ref,
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500",
                    bind:value=input_value,
                    on:blur=move |_| handle_blur(),
                    on:keyup=handle_keyup,
                )
            }))
        }
    }
}

#[component]
pub fn List() -> View {
    let app_state = use_context::<AppState>();
    let todos_left = create_selector(move || app_state.todos_left());

    let filtered_todos = create_memo(move || {
        app_state
            .todos
            .get_clone()
            .iter()
            .filter(|todo| match app_state.filter.get() {
                Filter::All => true,
                Filter::Active => !todo.with(|todo| todo.completed),
                Filter::Completed => todo.with(|todo| todo.completed),
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    // We need a separate signal for checked because clicking the checkbox will detach the binding
    // between the attribute and the view.
    let checked = create_signal(false);
    create_effect(move || {
        // Calling checked.set will also update the `checked` property on the input element.
        checked.set(todos_left.get() == 0)
    });

    view! {
        section(class="divide-y divide-gray-200") {
            input(
                id="toggle-all",
                class="sr-only",
                r#type="checkbox",
                readonly=true,
                bind:checked=checked,
                on:input=move |_| app_state.toggle_complete_all()
            )
            label(r#for="toggle-all", class="hidden")

            ul(class="divide-y divide-gray-200") {
                Keyed(
                    list=filtered_todos,
                    view=|todo| view! {
                        Item(todo=todo)
                    },
                    key=|todo| todo.with(|todo| todo.id),
                )
            }
        }
    }
}

#[component(inline_props)]
pub fn TodoFilter(filter: Filter) -> View {
    let app_state = use_context::<AppState>();
    let selected = move || filter == app_state.filter.get();
    let set_filter = move |filter| app_state.filter.set(filter);

    view! {
        li {
            a(
                class=if selected() {
                    "px-3 py-1 rounded-md bg-blue-500 text-white"
                } else {
                    "px-3 py-1 rounded-md hover:bg-gray-200 text-gray-600"
                },
                href=filter.url(),
                on:click=move |_| set_filter(filter),
            ) {
                (format!("{filter:?}"))
            }
        }
    }
}

#[component]
pub fn Footer() -> View {
    let app_state = use_context::<AppState>();

    let items_text = move || match app_state.todos_left() {
        1 => "item",
        _ => "items",
    };

    let has_completed_todos =
        create_selector(move || app_state.todos_left() < app_state.todos.with(Vec::len));

    let handle_clear_completed = move |_| app_state.clear_completed();

    view! {
        footer(class="flex items-center justify-between px-4 py-3 bg-gray-50 text-sm") {
            span(class="text-gray-600") {
                strong(class="font-medium") { (app_state.todos_left()) }
                span { " " (items_text) " left" }
            }
            ul(class="flex space-x-2") {
                TodoFilter(filter=Filter::All)
                TodoFilter(filter=Filter::Active)
                TodoFilter(filter=Filter::Completed)
            }

            (has_completed_todos.get().then(|| view! {
                button(
                    class="text-gray-500 hover:text-gray-700",
                    on:click=handle_clear_completed
                ) {
                    "Clear completed"
                }
            }))
        }
    }
}
