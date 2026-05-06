use dioxus::prelude::*;

use crate::aboutme::AboutMe;
use crate::colors::{BLUE, TEXT};
use crate::footer::Footer;
use crate::header::Header;
use crate::links::Links;
use crate::projects::Projects;
use crate::section::Section;

static BACKGROUND: Asset = asset!("/assets/bg.jpeg");
static PROFILE: Asset = asset!("/assets/me.png");

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            background_image: "url({BACKGROUND})",
            background_repeat: "no-repeat",
            background_size: "100% auto",
            background_position: "0 13em",
            div { margin: "10%",
                //Header {}
                div { display: "flex", margin_bottom: "17em",
                    div {
                        width: "7em",
                        height: "10em",
                        min_width: "6em",
                        overflow: "hidden",
                        border: "0.3em solid",
                        border_color: BLUE,
                        border_radius: "3.2em",
                        background_color: TEXT,
                        img {
                            src: PROFILE,
                            width: "100%",
                            height: "100%",
                            object_fit: "cover",
                        }
                    }
                    h1 { width: "fit-content", margin_left: "1em",
                        "Hi, I'm Flint, a sophomore studying computer science at Stony Brook University."
                    }
                }
                Section { name: "Projects", id: "projects", Projects {} }
                Section { name: "About Me", id: "about-me", AboutMe {} }
                for _ in 0..100 {
                    br {}
                }
                //Section { name: "links", Links {} }
                //Footer {}
            }
        }
    }
}
