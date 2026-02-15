use crate::state;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_timers::callback::Interval;

#[component]
pub fn NextExam(next: state::Event) -> Element {
    let mut remaining = use_signal(|| next.time - Utc::now());

    let mut interval_handle = use_signal(|| None::<Interval>);
    use_effect(move || {
        let target = next.time.clone();
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
        section {
            class: "rounded-2xl bg-primary px-6 py-12 text-center md:py-20",
            p {
                class: "text-sm uppercase tracking-widest text-primary-foreground/60",
                "Nächster Termin"
            }
            h2 {
                class: "mt-3 text-3xl font-display font-bold text-primary-foreground md:text-5xl text-balance",
                { next.name }
            }

            div {
                class: "mx-auto mt-10 flex items-center justify-center gap-3 md:gap-6 lg:gap-8",
                Section {
                    val: days,
                    desc: "Tage"
                }
                Seperator {}
                Section {
                    val: hours,
                    desc: "Stunden"
                }
                Seperator {}
                Section {
                    val: minutes,
                    desc: "Minuten"
                }
                Seperator {}
                Section {
                    val: seconds,
                    desc: "Sekunden"
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
                class: "text-5xl font-display font-bold tabular-nums leading-none text-primary-foreground md:text-7xl lg:text-8xl",
                { val.to_string() }
            }
            span {
                class: "mt-2 text-xs uppercase tracking-widest text-primary-foreground/70 md:text-sm",
                { desc }
            }
        }
    }
}

#[component]
pub fn Seperator() -> Element {
    rsx! {
        span {
            class: "text-3xl font-light text-primary-foreground/40 md:text-5xl lg:text-6xl",
            ":"
        }
    }
}
