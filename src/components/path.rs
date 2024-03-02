use leptos::*;

use crate::paths::{Path, PATHS};

use std::time::Duration;

#[component]
pub fn PathComponent(path: Path) -> impl IntoView {
    let (shown_text, set_shown_text) = create_signal("".to_string());
    let (count, set_count) = create_signal(0);

    let handle = set_interval_with_handle(
        move || {
            set_shown_text(
                shown_text().to_string()
                    + &format!(
                        "{}",
                        path.text.chars().nth(count()).expect("out of bounds!")
                    ),
            );
            set_count(count() + 1);
        },
        Duration::from_millis(70),
    )
    .expect("failed to set interval");

    create_effect(move |_| {
        if count() == path.text.len() {
            handle.clear()
        }
    });

    view! {
        <h1>{path.path}</h1>
        <main>
            <p>{shown_text}</p>
            <ul>
                <For each=move || path.choices.clone()
                     key=|choice| *choice
                     children=|choice| view! {
                                           <li><a href={choice}>{PATHS[choice].cta}</a></li>
                                       } />
            </ul>
        </main>

    }
}
