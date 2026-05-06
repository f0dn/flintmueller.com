use dioxus::prelude::*;

use crate::icons::GITHUB;

#[component]
pub fn Links() -> Element {
    rsx! {
        div {
            align_items: "center",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            Link {
                name: "GitHub",
                link: "https://github.com/f0dn",
                img: GITHUB,
            }
        }
    }
}

#[component]
fn Link(name: &'static str, link: &'static str, img: Asset) -> Element {
    rsx! {
        div { display: "flex", align_items: "center",
            img {
                src: img,
                alt: name,
                height: "15%",
                width: "15%",
                padding_right: "2%",
            }
            a { padding_top: "1%", href: link, {name} }
        }
    }
}
