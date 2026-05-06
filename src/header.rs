use dioxus::prelude::*;
use dioxus_feather_icons::prelude::*;

use crate::colors::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            height: "3em",
            background_color: BACKGROUND,
            display: "flex",
            justify_content: "center",
            width: "100%",
            position: "sticky",
            top: "0",
            SVGLink { link: "https://github.com/f0dn", icon: icon!(github) }
            SVGLink {
                link: "https://www.instagram.com/f0d1n",
                icon: icon!(instagram),
            }
            SVGLink {
                link: "https://linkedin.com/in/flint-mueller",
                icon: icon!(linkedin),
            }
            SVGLink { link: "RESUME", icon: icon!(file_text) }
        }
    }
}

#[component]
fn SVGLink(link: &'static str, icon: Element) -> Element {
    rsx! {
        div { padding: "0.5em", width: "2em",
            Link { to: link, {icon} }
        }
    }
}
