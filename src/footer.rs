use dioxus::prelude::*;

use crate::colors::*;

pub const FOOTER_HEIGHT: &str = "5%";

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            height: FOOTER_HEIGHT,
            background_color: BASE,
            font_size: "1.5em",
            display: "flex",
            justify_content: "center",
            width: "100%",
            position: "fixed",
            bottom: "0",
            FooterItem {
                name: "GitHub",
                link: "https://github.com/f0dn",
                svg: "github/github-mark-white.svg"
            }
        }
    }
}

#[component]
fn FooterItem(name: &'static str, link: &'static str, svg: &'static str) -> Element {
    rsx! {
        div {
            a { href: link,
                img { height: "100%", src: svg, alt: name }
            }
        }
    }
}
