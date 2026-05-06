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
            background_position: "0 10%",
            div {
                margin: "10%",
                //Header {}
                div { display: "flex",
                    div {
                        width: "200px",
                        height: "200px",
                        overflow: "hidden",
                        border: "5px solid",
                        border_color: BLUE,
                        border_radius: "60px",
                        background_color: TEXT,
                        img {
                            src: PROFILE,
                            width: "100%",
                            height: "100%",
                            object_fit: "cover",
                        }
                    }
                    h1 {
                        margin_left: "40px",
                        "Hi, I'm Flint, a sophomore studying computer science at Stony Brook University."
                    }
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
}
