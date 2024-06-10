#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::view::{blog::Blog, home::Home, todos::TodoListApp, todo_app::TodoAppApp};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/todos")]
    TodoListApp {},

    #[route("/todo_app")]
    TodoAppApp {},
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
