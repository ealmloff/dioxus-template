use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use ui::Hero;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting app.");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Hero {}
    }
}
