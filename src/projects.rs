use dioxus::prelude::*;
use dioxus_feather_icons::icon;

use crate::container::Container;

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
            Project {
                name: "Trinity Chess Engine",
                desc: "Experimental chess engine to play around with bitboards and search algorithms",
                link: "https://github.com/f0dn/chess",
            }
            Project {
                name: "Trinity Chess Engine",
                desc: "Experimental chess engine to play around with bitboards and search algorithms",
                link: "https://github.com/f0dn/chess",
            }
            Project {
                name: "Trinity Chess Engine",
                desc: "Experimental chess engine to play around with bitboards and search algorithms",
                link: "https://github.com/f0dn/chess",
            }
            Project {
                name: "Trinity Chess Engine",
                desc: "Experimental chess engine to play around with bitboards and search algorithms",
                link: "https://github.com/f0dn/chess",
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
fn Project(name: &'static str, desc: &'static str, link: &'static str) -> Element {
    rsx! {
        Container { margin_bottom: "1em", // to emulate row gap
            h2 { margin_top: "0.2em",
                Link { to: link, {name} }
            }
            p { {desc} }
        }
    }
}
