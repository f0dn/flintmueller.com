use dioxus::prelude::*;

#[component]
pub fn Section(name: &'static str, id: &'static str, children: Element) -> Element {
    rsx! {
        div { id, width: "100%",
            h1 { display: "flex", justify_content: "center", {name} }
            {children}
        }
    }
}
