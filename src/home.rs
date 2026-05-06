use dioxus::prelude::*;

use crate::aboutme::AboutMe;
use crate::colors::{BLUE, TEXT};
use crate::header::Header;
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
            background_position: "0 20vw",
            Header {}
            div { margin: "10%", margin_top: "0",
                div {
                    display: "flex",
                    align_items: "center",
                    height: "28vw",
                    margin_bottom: "27vw",
                    div {
                        width: "16vw",
                        height: "20vw",
                        overflow: "hidden",
                        border: "0.7vw solid",
                        border_color: BLUE,
                        border_radius: "7vw",
                        background_color: TEXT,
                        img {
                            src: PROFILE,
                            width: "100%",
                            height: "100%",
                            object_fit: "cover",
                        }
                    }
                    h1 {
                        width: "fit-content",
                        margin_left: "1em",
                        font_size: "3.5vw",
                        "Hi, I'm Flint, studying computer science at Stony Brook University."
                    }
                }
                Section { name: "Top Projects", id: "projects", Projects {} }
                Section { name: "About Me", id: "about-me", AboutMe {} }
            }
        }
        {dioxus_feather_icons::sprite!()}
    }
}
