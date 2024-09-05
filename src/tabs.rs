use dioxus::prelude::*;

#[component]
fn Styled(children: Element) -> Element {
    rsx! {
       head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
       {children}
    }
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabItem {
    #[props(into)]
    pub id: String,
    pub children: Element,
}

#[derive(Clone, Default, PartialEq, Props)]
pub struct Props {
    #[props(into)]
    default_active_id: Option<String>,
    items: Vec<TabItem>,
}

#[component]
fn TabPane(active_id: Signal<String>, children: Element) -> Element {
    children
}

#[component]
#[allow(clippy::missing_errors_doc)]
pub fn Tabs(props: Props) -> Element {
    let mut active_id: Signal<String> = use_signal(|| {
        props
            .default_active_id
            .or_else(|| props.items.first().map(|i| i.id.clone()))
            .unwrap_or_default()
    });

    let default_class = "whitespace-nowrap border-b-2 border-transparent px-1 py-4 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700";
    let active_class = "whitespace-nowrap border-b-2 border-indigo-500 px-1 py-4 text-sm font-medium text-indigo-600";

    let item: Option<TabItem> = props
        .items
        .iter()
        .find(|i| i.id == *active_id.read_unchecked())
        .cloned();

    rsx! {
        Styled {
            div {
                div { class: "hidden sm:block",
                    div { class: "border-b border-gray-200",
                        nav { "aria-label": "Tabs", class: "-mb-px flex space-x-8",
                            for tab in props.items {
                                a {
                                    key: "{tab.id}",
                                    href: "#",
                                    class: if tab.id == *active_id.read() { active_class } else { default_class },
                                    onclick: move |_| {
                                        active_id.write().clone_from(&tab.id);
                                    },
                                    "{tab.id}"
                                }
                            }
                        }
                    }
                }

                if let Some(item) = item {
                    TabPane { active_id, children: item.children }
                }
            }
        }
    }
}
