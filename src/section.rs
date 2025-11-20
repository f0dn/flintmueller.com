use dioxus::prelude::*;

#[component]
pub fn Section(name: &'static str, children: Element) -> Element {
    rsx! {
        div { id: "{name}",
            h2 { "{name}" }
            {children}
        }
    }
}
