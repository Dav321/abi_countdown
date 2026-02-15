mod components;
mod state;

use crate::components::{Card, NextExam, Progress};
use chrono::Utc;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

fn main() {
    dioxus::logger::init(Level::INFO).unwrap();

    launch(|| {
        rsx! {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    h1 {
                        "Loading"
                    }
                },
                App {}
            }
        }
    })
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/tailwind.css");
const EVENTS: Asset = asset!("assets/events.json");

#[component]
fn App() -> Element {
    let events = use_resource(|| async move {
        let bytes = dioxus::asset_resolver::read_asset_bytes(&EVENTS)
            .await
            .unwrap();
        let events: Vec<state::Event> = serde_json::from_slice(&bytes).unwrap();
        events
    })
    .suspend()?;
    let event_count = events.len();

    let todo: Vec<state::Event> = events
        .iter()
        .map(|e| e.cloned())
        .filter(|e| e.time > Utc::now())
        .collect();
    let todo_count = todo.len();

    let next = if todo_count > 0 {
        todo[0].clone()
    } else {
        state::Event {
            name: "Fertig!".to_string(),
            flag: "".to_string(),
            time: Utc::now(),
            color: "".to_string(),
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: CSS }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300..700&display=swap" }

        main {
            class: "min-h-screen bg-backgroud",
            div {
                class: "mx-auto max-w-7xl px-4 py-10 md:px-6 md:py-16",
                header {
                    class: "mb-10 md:mb-14",
                    h1 {
                        class: "text-4xl font-display font-bold tracking-tight text-foreground md:text-5xl text-balance",
                        "Abitur 2026"
                    }
                    p {
                        class: "mt-3 text-lg text-muted-foreground",
                        "Countdown zu allen Klausuren."
                    }
                }

                NextExam {
                    next
                }

                div {
                    class: "mt-8",
                    Progress {
                        completed: event_count - todo_count,
                        total: event_count
                    }
                }

                section {
                    class: "mt-10 md:mt-14",
                    h2 {
                        class: "mb-6 text-lg font-display font-semibold text-foreground",
                        "Alle Termine"
                    }
                    div {
                        class: "grid gap-4 md:grid-cols-2",
                        for event in events.iter() {
                            Card {
                                event: event.cloned()
                            }
                        }
                    }
                }
            }
        }
    }
}
