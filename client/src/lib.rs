use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use dill::CatalogBuilder;
use dioxus::prelude::*;

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
    rsx! {
        section {
            h2 { "个人资料" }
            p { "账号：demo" }
            p { "当前会话绑定默认租户。" }
        }
    }
}
