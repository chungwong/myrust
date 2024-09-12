#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dioxus_logger::init(tracing::Level::INFO)?;
    launch(App);
    Ok(())
}

fn App() -> Element {
    rsx! {
        form {
            Item {
                Input {
                    name: "first name",
                }
            }

            Item {
                Input {
                    name: "last name",
                }
            }
        }
    }
}

#[component]
fn Styled(children: Element) -> Element {
    rsx! {
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
        {children}
    }
}

#[component]
fn Item(children: Element) -> Element {
    rsx! {
        Styled {
            { children }
        }
    }
}

#[component]
fn Input(name: String, value: Option<String>) -> Element {
    rsx! {
        Styled {
            input {
                name,
                value,
            }
        }
    }
}
