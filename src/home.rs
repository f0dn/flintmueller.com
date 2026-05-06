use dioxus::prelude::*;

use crate::aboutme::AboutMe;
use crate::footer::Footer;
use crate::header::Header;
use crate::links::Links;
use crate::projects::Projects;
use crate::section::Section;

static BACKGROUND: Asset = asset!("/assets/bg.jpeg");

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            background_image: "url({BACKGROUND})",
            background_repeat: "no-repeat",
            background_size: "100% auto",
            background_position: "0 10%",
            //Header {}
            div {
                h1 { "Hi, I'm Flint, a sophomore studying computer science at Stony Brook University." }
            }
            Section { name: "projects", Projects {} }
            Section { name: "about me", AboutMe {} }
            for _ in 0..100 {
                br {}
            }
            //Section { name: "links", Links {} }
            //Footer {}
        }
    }
}
