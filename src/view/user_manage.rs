#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub birthday: String,
}

pub type Users = HashMap<i32, User>;

pub fn UserManage() -> Element {
    // management state
    let mut users = use_signal(|| Users::new());
    let _id = use_signal(|| 0);

    rsx! {
        h2 {
            "User Management"
        }

        div {
            AddUserForm {
                id_signal: _id,
                user_signal: users,
            }
        }

        div {
            UserTable {
                users: users,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct AddUserProps {
    pub id_signal: Signal<i32>,
    pub user_signal: Signal<Users>,
}

#[component]
pub fn AddUserForm(mut props: AddUserProps) -> Element {
    // input field control state
    // let mut input_id = use_signal(|| 0);
    let mut input_username = use_signal(|| "".to_string());
    let mut input_email = use_signal(|| "".to_string());
    let mut input_birthday = use_signal(|| "".to_string());

    rsx! {
        form {
            onsubmit: move |ev: FormEvent| {
                tracing::info!("Form submitted: {:?}", ev);
                if input_username().is_empty() || input_email().is_empty() || input_birthday().is_empty() {
                    return;
                }
                props.id_signal += 1;

                let user = User {
                    id: *props.id_signal.read(),
                    username: input_username(),
                    email: input_email(),
                    birthday: input_birthday(),
                };

                props.user_signal.write().insert(*props.id_signal.read(), user);

                input_username.set("".to_string());
                input_email.set("".to_string());
                input_birthday.set("".to_string());

                tracing::info!("Users: {:?}", props.user_signal);
            },

            div {
                label {
                    r#for: "input_username",
                    "Username"
                }
                input {
                    r#type: "text",
                    placeholder: "Username",
                    value: "{input_username}",
                    id: "input_username",
                    name: "input_username",
                    oninput: move |ev| input_username.set(ev.value().clone()),
                }
            }

            div {
                label {
                    r#for: "input_email",
                    "Email"
                }
                input {
                    r#type: "text",
                    placeholder: "Email",
                    value: "{input_email}",
                    id: "input_email",
                    name: "input_email",
                    oninput: move |ev| input_email.set(ev.value().clone()),
                }
            }

            div {
                label {
                    r#for: "input_birthday",
                    "Birthday"
                }
                input {
                    r#type: "text",
                    placeholder: "Birthday",
                    value: "{input_birthday}",
                    id: "input_birthday",
                    name: "input_birthday",
                    oninput: move |ev| input_birthday.set(ev.value().clone()),
                }
            }

            div {
                input {
                    r#type: "submit",
                    name: "submit",
                    value: "Add User",
                }
            }

        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UserRowProps {
    pub id: i32,
    pub user_signal: Signal<Users>,
}

#[component]
pub fn UserRow(props: UserRowProps) -> Element {
    let users = props.user_signal.read();
    let user = &users[&props.id];

    rsx! {
        tr {
            td {
                class: "border px-4 py-2",
                button {
                    "Edit"
                }
            }
            td { class: "border px-4 py-2", "{user.id}" }
            td { class: "border px-4 py-2", "{user.username}" }
            td { class: "border px-4 py-2", "{user.email}" }
            td { class: "border px-4 py-2", "{user.birthday}" }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UserTableProps {
    users: Signal<Users>,
}

#[component]
pub fn UserTable(props: UserTableProps) -> Element {
    rsx! {
        table {
            class: "table-auto",
            tr {
                th { class: "px-4 py-2", "Action" }
                th { class: "px-4 py-2", "ID" }
                th { class: "px-4 py-2", "Username" }
                th { class: "px-4 py-2", "Email" }
                th { class: "px-4 py-2", "Birthday" }
            }
            for (id, _) in &*props.users.read() {
                UserRow {
                    id: *id,
                    user_signal: props.users,
                }
            }
        }
    }
}
