#![allow(non_snake_case)]

use dioxus::prelude::*;

mod aboutme;
mod colors;
mod container;
mod footer;
mod header;
mod home;
mod icons;
mod links;
mod projects;
mod section;

use home::Home;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
