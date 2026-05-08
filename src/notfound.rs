use dioxus::prelude::*;

use crate::colors::TEXT;
use crate::Route;

#[component]
pub fn NotFound() -> Element {
    rsx! {
        div { color: TEXT, height: "100vh", align_content: "center",
            h1 { "404" }
            "theres nothing here :("
            br {}
            "do you want the "
            Link { to: Route::Home, "home page" }
            "?"
            br {}
            small { "or maybe i have a broken link (shhh)" }
        }
    }
}
