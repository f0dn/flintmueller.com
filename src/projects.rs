use dioxus::prelude::*;

use crate::colors::*;
use crate::header::Header;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Header {}
        div {
            Project {
                name: "This website",
                desc: "A website built with Dioxus, a Rust web framework.",
                link: "/"
            }
        }
    }
}

#[component]
fn Project(name: &'static str, desc: &'static str, link: &'static str) -> Element {
    rsx! {
        div { background_color: BLUE_6, color: TEXT, padding: "2%",
            h1 {
                Link { to: link, "{name}" }
            }
            p { "{desc}" }
        }
    }
}
