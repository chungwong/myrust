#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let open = use_signal(|| false);

    rsx! {
        Drawer {
            open,
        }
    }
}

#[component]
fn Drawer(open: Signal<bool>) -> Element {
    // set overflow of body according to the state of `open`
    use_effect(move || {
        tracing::info!("drawer effect");
        body_overflow(open());
    });

    // reset overflow to auto when this Drawer is destroyed
    use_drop(move || {
        tracing::info!("dropping");
        body_overflow(false);
    });

    rsx! {
        if open() {
            div {
                "I am a drawer"
            }
        }
    }
}

/// set overflow property on body
fn body_overflow(open: bool) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let overflow = if open {
                    "overflow: hidden"
                } else {
                    "overflow: auto"
                };

                let _ = body.set_attribute("style", overflow);
            }
        }
    }
}
