use dioxus::prelude::*;

#[component]
pub fn Section(name: &'static str, children: Element) -> Element {
    rsx! {
        div { id: name, width: "100%",
            h2 { display: "flex", justify_content: "center", {name} }
            {children}
        }
    }
}
