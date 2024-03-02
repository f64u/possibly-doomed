use leptos::*;

use crate::paths::{Path, PATHS};

use std::time::Duration;

#[component]
pub fn PathComponent(path: Path) -> impl IntoView {
    let (shown_text, set_shown_text) = create_signal("".to_string());
    let (count, set_count) = create_signal(0);
    let (finished, set_finished) = create_signal(false);

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
        Duration::from_millis(30),
    )
    .expect("failed to set interval");

    create_effect(move |_| {
        if count() == path.text.len() {
            handle.clear();
            set_finished(true);
        }
    });

    let scanned_path = path
        .path
        .split("/")
        .scan("/".to_string(), |state, newpart| {
            log::info!("state: {state}, newpart: {newpart}");

            if *state == "/" {
                *state = "".into();
                Some(view! {<a class="hover:text-slate-500 hover:cursor-pointer" href="/">"/"</a>}.into_view())
            } else {
                *state = format!("{}/{}", *state, newpart);
                Some(
                    view! {<a class="hover:text-slate-500 hover:cursor-pointer" href={&*state}>{newpart}</a>}
                        .into_view(),
                )
            }
        })
        .collect::<Vec<_>>();

    let mut scanned_path_aug = Vec::new();
    let len = scanned_path.len();
    for (i, p) in scanned_path.into_iter().enumerate() {
        scanned_path_aug.push(p);
        if i != 0 && i != len - 1 {
            scanned_path_aug.push(view! {<span key={i}>"/"</span>}.into_view())
        }
    }

    view! {
        <main class="flex flex-col items-start mx-40">
            <h1 class="text-5xl font-bold self-center">{scanned_path_aug}</h1>
            <p class="text-left" style="white-space: pre-line">{shown_text}</p>
                <ul class="text-left">
                    <For each=move || path.choices.clone()
                         key=|choice| *choice
                         children=move |choice|
                                    view! {
                                    <Show when=finished>
                                       <li class="m-0 p-0 hover:cursor-pointer">
                                        <a class="text-slate-400 hover:text-sky-400" href={choice}>{PATHS[choice].cta}</a>
                                       </li>
                                    </Show>
                                    } />
                </ul>
        </main>

    }
}
