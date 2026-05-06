use dioxus::prelude::*;

use crate::container::Container;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div {
            display: "grid",
            justify_content: "center",
            grid_template_columns: "repeat(var(--project_columns, 3), auto)",
            gap: "1em",
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
            h2 {
                margin_top: "0.2em",
                Link { to: link, {name} }
            }
            p { {desc} }
        }
    }
}
