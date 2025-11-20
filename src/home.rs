use dioxus::prelude::*;

use crate::header::Header;
use crate::links::Links;
use crate::projects::Projects;
use crate::section::Section;

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}
        div {
            h2 { "Hi, I'm Flint," }
            "a senior at Stuyvesant High School."
            br {}
            "Check out my projects "
            Link { to: "#projects", "here" }
            "."
        }
        Section { name: "projects", Projects {} }
        Section { name: "links", Links {} }
    }
}
