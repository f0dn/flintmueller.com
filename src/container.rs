use crate::colors::*;
use dioxus::prelude::*;

#[component]
pub fn Container(children: Element) -> Element {
    rsx! {
        div {
            background_color: BASE,
            color: TEXT,
            padding: "2%",
            border: "3px solid",
            border_color: BLUE,
            {children}
        }
    }
}
