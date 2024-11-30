use dioxus::desktop::Config;
use dioxus::prelude::*;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_menu(None)) // remove the default menu
        .launch(App)
}

#[component]
fn App() -> Element {
    rsx! {}
}
