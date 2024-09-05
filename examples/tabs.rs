#![allow(non_snake_case)]

use dioxus::prelude::*;
use ui::tabs::{TabItem, Tabs};

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        Tabs {
            default_active_id: "Tab 1",
            items: vec![
                TabItem {
                    id: "Tab 1".to_string(),
                    children: rsx! {
                        "tab 1 content"
                    },
                },
                TabItem {
                    id: "Tab 2".to_string(),
                    children: rsx! {
                        "tab 2 content"
                    },
                },
            ]
        }
    }
}
