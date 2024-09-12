use dioxus::prelude::*;
use std::error::Error;
use ui::datepicker::DatePicker;

#[component]
fn Home() -> Element {
    rsx! {
        "Home"
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(Template)]
        #[route("/")]
        Home {},
        #[route("/test")]
        Test { },
}

#[component]
fn Test() -> Element {
    rsx! {
        DatePicker { }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div { class: "hidden lg:block lg:flex-1",
            "Hello World"
        }
        Link {
            to: Route::Test { },
            "A Link"
        }
    }
}

#[component]
fn Template() -> Element {
    rsx! {
        div {
            Header {  }
            main { Outlet::<Route> {} }
        }
    }
}

#[component]
pub(crate) fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dioxus_logger::init(tracing::Level::INFO)?;
    launch(App);
    Ok(())
}
