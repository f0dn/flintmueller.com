use crate::colors::*;
use dioxus::prelude::*;

#[component]
pub fn Container(
    children: Element,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {
            background_color: BASE,
            color: TEXT,
            padding: "1em",
            border: "0.3em solid",
            border_color: BLUE,
            ..attributes,
            {children}
        }
    }
}
