use dioxus::prelude::*;

use crate::colors::*;

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
                name: "This Website",
                desc: "A website built with Dioxus, a Rust web framework.",
                link: "#"
            }
            Project {
                name: "Rust Compiler",
                desc: "A compiler for a C-like language written in Rust.",
                link: "https://github.com/f0dn/razor"
            }
            Project {
                name: "Stuy Fission Robot Code",
                desc: "2024-25 robot code for 310 Stuy Fission (Stuy's FTC team)",
                link: "https://github.com/Fission310/into-the-deep"
            }
        }
    }
}

#[component]
fn Project(name: &'static str, desc: &'static str, link: &'static str) -> Element {
    rsx! {
        div { background_color: BLUE, color: TEXT, padding: "2%",
            h1 {
                Link { to: link, "{name}" }
            }
            p { "{desc}" }
        }
    }
}
