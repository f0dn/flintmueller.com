use dioxus::prelude::*;

use crate::header::Header;

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {},
        "Hi, it's Flint!"
    }
}
