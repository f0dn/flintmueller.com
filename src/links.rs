use dioxus::prelude::*;

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
                svg: "github/github-mark-white.svg"
            }
        }
    }
}

#[component]
fn Link(name: &'static str, link: &'static str, svg: &'static str) -> Element {
    rsx! {
        div { display: "flex", align_items: "center",
            img {
                src: svg,
                alt: name,
                height: "15%",
                width: "15%",
                padding_right: "2%"
            }
            a { padding_top: "1%", href: link, "{name}" }
        }
    }
}
