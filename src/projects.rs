use dioxus::prelude::*;

use crate::container::Container;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            padding: "15px",
            display: "grid",
            justify_content: "center",
            grid_template_columns: "repeat(auto-fit, minmax(200px, 300px))",
            gap: "15px",
            Project {
                name: "Razor",
                desc: "Custom build compiler written in rust",
                link: "https://github.com/f0dn/razor",
            }
            Project {
                name: "This Website",
                desc: "My personal portfolio written with Dioxus",
                link: "#",
            }
            Project {
                name: "Stuy Fission Robot Code",
                desc: "2024-25 robot code for 310 Stuy Fission (Stuy's FTC team)",
                link: "https://github.com/Fission310/into-the-deep",
            }
            Project {
                name: "Trinity Chess Engine",
                desc: "Experimental chess engine to play around with bitboards and search algorithms",
                link: "https://github.com/f0dn/chess",
            }
        }
    }
}

#[component]
fn Project(name: &'static str, desc: &'static str, link: &'static str) -> Element {
    rsx! {
        Container {
            h1 {
                Link { to: link, "{name}" }
            }
            p { "{desc}" }
        }
    }
}
