use aio_plugin_identity_model::{IdentityErrorResponse, IdentityResponse, LoginRequest};
use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use az_ui_components::{button::Button, input::Input};
use dill::CatalogBuilder;
use dioxus::prelude::*;

pub use aio_plugin_identity_model::SessionView;

#[derive(Debug)]
pub struct IdentityPlugin;

impl ApplicationPlugin for IdentityPlugin {
    fn pages(&self) -> Vec<ApplicationPage> {
        vec![ApplicationPage {
            id: "profile",
            label: "个人资料",
            icon: Some("user"),
            scene: ApplicationScene {
                id: "system",
                label: "系统",
            },
            render: ProfilePage,
        }]
    }
}

pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(IdentityPlugin)
        .bind::<dyn ApplicationPlugin, IdentityPlugin>();
}

#[allow(non_snake_case)]
fn ProfilePage() -> Element {
    let session = use_resource(load_session);
    rsx! {
        section {
            h2 { "个人资料" }
            match session.read().as_ref() {
                Some(Ok(Some(session))) => rsx! {
                    p { "账号：{session.account}" }
                    p { "姓名：{session.display_name}" }
                    p { "当前租户：{session.tenant_label}" }
                },
                Some(Ok(None)) => rsx! { p { role: "alert", "会话已失效" } },
                Some(Err(error)) => rsx! { p { role: "alert", "读取会话失败：{error}" } },
                None => rsx! { p { "正在读取会话" } },
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn LoginPage() -> Element {
    let mut account = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut pending = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    rsx! {
        main { class: "min-h-screen grid place-items-center p-4",
            article { class: "border p-6 w-full max-w-sm",
                h1 { "AIO" }
                p { "登录到你的工作区" }
                form {
                    class: "grid gap-3",
                    onsubmit: move |event| {
                        event.prevent_default();
                        let request = LoginRequest {
                            account: account(),
                            password: password(),
                        };
                        pending.set(true);
                        error.set(None);
                        spawn(async move {
                            match login(request).await {
                                Ok(()) => reload(),
                                Err(message) => error.set(Some(message)),
                            }
                            pending.set(false);
                        });
                    },
                    label { r#for: "account", "账号" }
                    Input {
                        id: "account",
                        name: "account",
                        autocomplete: "username",
                        aria_label: "账号",
                        value: account(),
                        oninput: move |event: FormEvent| account.set(event.value()),
                    }
                    label { r#for: "password", "密码" }
                    Input {
                        id: "password",
                        name: "password",
                        r#type: "password",
                        autocomplete: "current-password",
                        aria_label: "密码",
                        value: password(),
                        oninput: move |event: FormEvent| password.set(event.value()),
                    }
                    if let Some(message) = error() {
                        p { role: "alert", "{message}" }
                    }
                    Button { r#type: "submit", disabled: pending(),
                        if pending() { "正在登录" } else { "登录" }
                    }
                }
            }
        }
    }
}

pub async fn load_session() -> Result<Option<SessionView>, String> {
    let response = gloo_net::http::Request::get("/api/auth/session")
        .send()
        .await
        .map_err(|error| error.to_string())?;
    if response.status() == 401 {
        return Ok(None);
    }
    if !response.ok() {
        return Err(response.text().await.unwrap_or_default());
    }
    response
        .json::<IdentityResponse<SessionView>>()
        .await
        .map(|response| Some(response.data))
        .map_err(|error| error.to_string())
}

async fn login(request: LoginRequest) -> Result<(), String> {
    let response = gloo_net::http::Request::post("/api/auth/login")
        .json(&request)
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;
    if response.ok() {
        return Ok(());
    }
    let body = response.text().await.unwrap_or_default();
    Err(serde_json::from_str::<IdentityErrorResponse>(&body)
        .map(|response| response.error)
        .unwrap_or(body))
}

fn reload() {
    if let Some(window) = web_sys::window() {
        let _ = window.location().reload();
    }
}
