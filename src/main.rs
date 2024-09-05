#![allow(non_snake_case)]

// mod tabs;

use dioxus::prelude::*;
use std::error::Error;
use tracing::Level;
use ui::tabs::{TabItem, Tabs};

fn App() -> Element {
    rsx! {
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }

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

// fn main() {
//     launch(App);
// }

fn main() -> Result<(), Box<dyn Error>> {
    dioxus_logger::init(Level::INFO)?;
    #[cfg(feature = "web")]
    {
        launch(App);
    }

    #[cfg(feature = "server")]
    {
        use axum::Router;
        use std::error::Error;

        tokio::runtime::Runtime::new()?.block_on(async move {
            let mut app = Router::new().serve_dioxus_application(ServeConfig::new()?, App);

            let address = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
            let listener = tokio::net::TcpListener::bind(&address).await?;

            axum::serve(listener, app.into_make_service()).await?;

            Ok::<(), Box<dyn Error>>(())
        })?;
    }
    Ok(())
}
