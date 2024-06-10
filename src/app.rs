#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::view::{blog::Blog, home::Home, todos::TodoListApp, todo_app::TodoAppApp, user_manage::UserManage};

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

    #[route("/user_manage")]
    UserManage {},
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
