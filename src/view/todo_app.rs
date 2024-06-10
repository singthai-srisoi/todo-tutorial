#![allow(non_snake_case)]

use dioxus::dioxus_core::Element;
use dioxus::events::keyboard_types::Key;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub task: String,
    pub checked: bool,
}

pub type Todos = HashMap<u32, TodoItem>;

pub fn TodoAppApp() -> Element {
    // todo list state
    let mut todos = use_signal(|| Todos::new());
    let mut _id: Signal<u32> = use_signal(|| 0);

    // input element state
    let mut input_task = use_signal(|| "".to_string());

    rsx! {
        h1 {
            class: "ml-6 mr-6 text-2xl font-bold text-gray-800",
            "Todo List"
        }
        div {
            class: "ml-6 mr-6",
            input {
                class: "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5",
                r#type: "text",
                name: "task",
                value: "{input_task}",
                autofocus: "true",
                oninput: move |ev| input_task.set(ev.value().clone()),
                onkeydown: move |ev| {
                    if ev.key() == Key::Enter && !input_task().is_empty() {
                        _id += 1;
                        todos.write().insert(_id(), TodoItem {
                            id: _id(),
                            task: input_task().clone(),
                            checked: false,
                        });
                        input_task.set("".to_string());
                        tracing::info!("Todos: {:?}", todos);
                    }
                }
            }
        }
        div {
            class: "ml-6 mr-6",
            for (id, _) in &*todos.read() {
                TodoEntry { set_todo: todos, id: *id }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Props)]
pub struct TodoEntryProps {
    pub set_todo: Signal<Todos>,
    pub id: u32,
}

#[component]
pub fn TodoEntry(mut props: TodoEntryProps) -> Element {
    // get the todo item from the signal
    let todos = props.set_todo.read();
    let todo = &todos[&props.id];

    // input element state
    let mut input_checked = use_signal(|| todo.checked);

    rsx! {
        div {
            class: "flex items-center",
            input {
                class: "border-2 border-gray-300 p-2",
                r#type: "checkbox",
                name: "checked",
                checked: "{input_checked}",
                onchange: move |ev| {
                    input_checked.set(ev.checked());
                    props.set_todo.write().get_mut(&props.id).unwrap().checked = input_checked();
                    tracing::info!("Todos: {:?}", props.set_todo);
                }
            }
            span {
                contenteditable: "true",
                class: "ml-2",
                "{todo.task}"
            }
        }
    }
}
