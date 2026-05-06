use crate::colors::*;
use dioxus::prelude::*;

#[component]
pub fn Container(children: Element) -> Element {
    rsx! {
        div {
            background_color: BASE,
            color: TEXT,
            padding: "1em",
            border: "0.3em solid",
            border_color: BLUE,
            {children}
        }
    }
}
