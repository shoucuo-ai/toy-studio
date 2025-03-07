use sycamore::prelude::*;
#[derive(Clone, PartialEq, Eq)]
struct Todo {
    task: String,
    id: u32,
}

#[component]
pub fn ListDemo() -> View {
    let todos = vec![
        Todo {
            task: "Learn Rust".to_string(),
            id: 1,
        },
        Todo {
            task: "Learn Sycamore".to_string(),
            id: 2,
        },
    ];

    view! {
        ul {
            Keyed(
              list = todos,
              view = |todo|  view! {
                li { (todo.task) }
              },
              key = |todo| todo.id
            )
        }
    }
}

#[component]
pub fn TodoItem(todo: Todo) -> View {
    view! {
        li { (todo.task) }
    }
}

#[component(inline_props)]
pub fn TodoList(todos: Vec<Todo>) -> View {
    view! {
        ul {
            Indexed(list = todos, view = TodoItem)
        }
    }
}

#[component]
pub fn TodoApp() -> View {
    let todos = vec![Todo {
        task: "Learn Rust".to_string(),
        id: 1,
    }];

    view! {
        TodoList(todos=todos)
    }
}
