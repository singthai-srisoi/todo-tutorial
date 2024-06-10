#![allow(non_snake_case)]

use dioxus::events::FormEvent;
use dioxus::prelude::*;
use tracing::info;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    id: usize,
    text: String,
    checked: bool,
}

pub fn TodoListApp() -> Element {
    let mut _id: Signal<usize> = use_signal(|| 0);
    let mut _todos = use_signal(|| Vec::<Todo>::new());

    // input field value
    let mut _input_name = use_signal(|| "".to_string());

    let on_submit = move |ev: FormEvent| {
        info!("Form submitted: {:?}", ev);
        _id += 1;
        _todos.push(Todo {
            id: _id(),
            text: _input_name(),
            checked: false,
        });
        _input_name.set("".to_string());
        info!("Todos: {:?}", _todos);
    };

    rsx! {
        form {
            onsubmit: on_submit,
            input {
                r#type: "text",
                name: "text",
                placeholder: "Add a new todo",
                value: "{_input_name}",
                oninput: move |ev| _input_name.set(ev.value().clone()),
            }
            input {
                r#type: "submit",
                name: "submit",
                value: "Add",
            }
            TodoItems { todo: _todos }
        }
    }
}

#[component]
pub fn TodoItems(todo: Signal<Vec<Todo>>) -> Element {
    rsx! {
        for t in &*todo.read() {
            div {
                input {
                    
                    r#type: "checkbox",
                    checked: t.checked,
                }
                span { "{t.text}" }
            }
        }
    }
}
