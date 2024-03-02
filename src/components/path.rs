use crate::path::Path;
use leptos::*;

/// A parameterized incrementing button
#[component]
pub fn PathComponent(path: Path) -> impl IntoView {
    view! {
        <h1>This is a path!</h1>
        <p>My text says: {path.text}</p>
    }
}
