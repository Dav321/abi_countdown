use dioxus::prelude::*;

#[component]
pub fn Progress(completed: usize, total: usize) -> Element {
    let percent = (completed as f32 / total as f32) * 100.0;

    rsx! {
        div {
            class: "rounded-2xl border border-border bg-card p-6 md:p-8",
            div {
                class: "flex items-center justify-between",
                div {
                    h3 {
                        class: "text-lg font-display font-semibold text-card-foreground",
                        "Fortschritt"
                    }
                    p {
                        class: "mt-0.5 text-sm text-muted-foreground",
                        "{completed} von {total} Terminen abgeschlossen"
                    }
                }
                span {
                    class: "text-2xl font-display font-bold text-card-foreground",
                    "{percent:.1}%"
                }
            }
            div {
                class: "mt-4 h-3 w-full overflow-hidden rounded-full bg-secondary",
                div {
                    class: "h-full rounded-full bg-primary transition-all duration-700 ease-out",
                    style: "width: {percent}%"
                }
            }
        }
    }
}
