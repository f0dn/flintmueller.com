use dioxus::prelude::*;

use crate::colors::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            background_color: BLUE_6,
            font_size: "1.5em",
            display: "flex",
            justify_content: "space-between",
            width: "100%",
            HeaderItem { path: "/", name: "home" }
            HeaderItem { path: "/projects", name: "projects" }
            HeaderItem { path: "/links", name: "links" }
        }
    }
}

#[component]
fn HeaderItem(path: &'static str, name: &'static str) -> Element {
    rsx! {
        div { padding: "0.5em",
            Link { to: path, "{name}" }
        }
    }
}
