use chrono::Utc;
use dioxus::prelude::*;
use gloo_timers::callback::Interval;

#[component]
pub fn Card(event: crate::state::Event) -> Element {
    let mut remaining = use_signal(|| event.time - Utc::now());

    let mut interval_handle = use_signal(|| None::<Interval>);
    use_effect(move || {
        let target = event.time.clone();
        let interval = Interval::new(1000, move || {
            remaining.set(target - Utc::now());
        });
        interval_handle.set(Some(interval));
    });

    let total_seconds = remaining().num_seconds().max(0);
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    rsx! {
        div {
            class: "group relative overflow-hidden rounded-2xl border border-border bg-card transition-all hover:shadow-lg",
            div {
                class: "h-1.5 w-full {event.color}"
            }
            div {
                class: "p-6 md:p-8",
                div {
                    class: "flex items-start justify-between",
                    div {
                        h2 {
                            class: "text-xl font-display font-bold text-card-foreground md:text-2xl",
                            { event.name }
                        }
                        p {
                            class: "mt-1 text-sm text-muted-foreground",
                            { event.time.format("%d.%m.%Y").to_string() }
                        }
                    }
                    if event.flag.len() > 0 {
                        span {
                            class: "inline-flex items-center rounded-full border border-border px-3 py-1 text-xs font-medium text-muted-foreground",
                            { event.flag }
                        }
                    }
                }

                div {
                    class: "mt-6",
                    if event.time < Utc::now() {
                        div {
                            class: "flex items-center justify-center rounded-xl bg-secondary py-6",
                            span {
                                class: "text-lg font-medium text-muted-foreground",
                                "Abgeschlossen!"
                            }
                        }
                    } else {
                        div {
                            class: "flex items-center justify-center gap-4 rounded-xl bg-secondary py-6 md:gap-8",
                            Section {
                                val: days,
                                desc: "Tage"
                            }
                            Seperator {}
                            Section {
                                val: hours,
                                desc: "Std"
                            }
                            Seperator {}
                            Section {
                                val: minutes,
                                desc: "Min"
                            }
                            Seperator {}
                            Section {
                                val: seconds,
                                desc: "Sek"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Section(val: i64, desc: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center",
            span {
                class: "text-3xl font-display font-bold tabular-nums leading-none md:text-5xl",
                { val.to_string() }
            }
            span {
                class: "mt-1.5 text-xs uppercase tracking-wider opacity-70 md:text-sm",
                { desc }
            }
        }
    }
}

#[component]
pub fn Seperator() -> Element {
    rsx! {
        span {
            class: "text-2xl font-light text-muted-foreground md:text-4xl",
            ":"
        }
    }
}
