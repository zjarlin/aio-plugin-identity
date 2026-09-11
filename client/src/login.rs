use super::http;
use aio_plugin_identity_model::LoginRequest;
use az_ui_components::{admin::StatusMessage, button::Button, input::Input};
use dioxus::prelude::*;

#[component]
pub fn LoginPage() -> Element {
    let mut account = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    rsx! {
        main { class: "admin-auth",
            section { class: "admin-auth-form", h1 { "AIO IDEA" } h2 { "账号登录" }
                form { class: "admin-form", onsubmit: move |event: FormEvent| {
                    event.prevent_default(); if busy() { return; } busy.set(true); error.set(None);
                    let payload = LoginRequest { account: account(), password: password() };
                    spawn(async move { if let Err(message) = http::login(payload).await { error.set(Some(message)); } busy.set(false); });
                },
                    fieldset { class: "admin-form", disabled: busy(),
                        label { class: "admin-field", span { "账号" } Input { name: "account", autocomplete: "username", aria_label: "账号", required: true, value: account(), oninput: move |event: FormEvent| account.set(event.value()) } }
                        label { class: "admin-field", span { "密码" } Input { name: "password", r#type: "password", autocomplete: "current-password", aria_label: "密码", required: true, value: password(), oninput: move |event: FormEvent| password.set(event.value()) } }
                    }
                    if let Some(message) = error() { StatusMessage { error: true, message } }
                    Button { r#type: "submit", disabled: busy(), if busy() { "正在登录" } else { "登录" } }
                }
            }
        }
    }
}
