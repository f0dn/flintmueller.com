use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            font_size: "1.5em",
            display: "flex",
            justify_content: "space-between",
            width: "100%",
            padding_bottom: "3%",
            HeaderItem { path: "/", name: "Home" }
            HeaderItem { path: "/about", name: "About" }
            HeaderItem { path: "/contact", name: "Contact" }
        }
    }
}

#[component]
fn HeaderItem(path: &'static str, name: &'static str) -> Element {
    rsx! {
        div {
            Link { to: path, "{name}" }
        }
    }
}
