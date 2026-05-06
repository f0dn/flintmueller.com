use dioxus::prelude::*;

use crate::footer::Footer;
use crate::header::Header;
use crate::links::Links;
use crate::projects::Projects;
use crate::aboutme::AboutMe;
use crate::section::Section;

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}
        div {
            h1 { "Hi, I'm Flint, a sophomore studying computer science at Stony Brook University." }
        }
        Section { name: "projects", Projects {} }
        Section { name: "about me", AboutMe {} }
        Section { name: "links", Links {} }
        Footer {}
    }
}
