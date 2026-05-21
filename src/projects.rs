use dioxus::prelude::*;
use dioxus_feather_icons::icon;

use crate::{colors::BLUE, container::Container};

#[component]
pub fn Projects() -> Element {
    let mut expanded = use_signal(|| false);

    rsx! {
        div {
            display: "grid",
            justify_content: "center",
            grid_template_columns: "repeat(var(--project_columns, 3), auto)",
            grid_template_rows: "auto auto",
            grid_auto_rows: if *expanded.read() { "auto" } else { "0" },
            overflow: "hidden",
            column_gap: "1em",
            Project { name: "Razor", link: "https://github.com/f0dn/razor",
                "Custom built compiler written in rust inspired by "
                Link { to: "https://www.youtube.com/playlist?list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs",
                    "this"
                }
                " wonderful YouTube series"
            }
            Project {
                name: "This Website",
                link: "https://github.com/f0dn/flintmueller.com",
                "My personal portfolio written built with "
                Link { to: "https://dioxuslabs.com", "Dioxus" }
            }
            Project {
                name: "Stuy Fission Robot Code",
                link: "https://github.com/Fission310/into-the-deep",
                "2024-25 robot code for 310 Stuy Fission (Stuyvesant's FTC team)"
            }
            Project {
                name: "Trinity Chess Engine",
                link: "https://github.com/f0dn/chess",
                "Experimental chess engine to play around with bitboards and search algorithms"
            }
            Project { name: "Reverse Engineering",
                "I've also done some experiments reverse engineering auth-token generation (not maliciously, just for fun!)"
            }
            Project {
                name: "Advent of Code",
                link: "https://github.com/f0dn/advent2025",
                "I complete "
                Link { to: "https://adventofcode.com", "Advent of Code" }
                " every year!"
            }
            Project {
                name: "QuackEngine",
                link: "https://github.com/f0dn/quackengine",
                "Chess engine for the "
                Link { to: "https://sbcs.io/project-quack", "Project Quack" }
                " group I mentored in my freshman year at Stony Brook"
            }
        }
        a {
            font_size: "1.3em",
            width: "fit-content",
            margin: "0 auto 3em",
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            href: "javascript:;",
            onclick: move |_| {
                expanded.toggle();
            },
            if !*expanded.read() {
                "Show More"
            }
            div { height: "2em", width: "2em",
                {if *expanded.read() { icon!(chevron_up) } else { icon!(chevron_down) }}
            }
            if *expanded.read() {
                "Show Less"
            }
        }
    }
}

#[component]
fn Project(
    name: &'static str,
    children: Element,
    #[props(into, default = "")] link: NavigationTarget, // a little hacky :O
) -> Element {
    rsx! {
        Container { margin_bottom: "1em", // to emulate row gap
            h2 { margin_top: "0.2em", color: BLUE,
                if link != Into::into("") {
                    Link { to: link, {name} }
                } else {
                    {name}
                }
            }
            p { {children} }
        }
    }
}
