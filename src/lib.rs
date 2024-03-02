#![feature(iter_intersperse)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;
mod paths;

use crate::components::path::PathComponent;
use crate::paths::PATHS;

macro_rules! routes {
    ($($route:expr),+) => {
        vec![$(view! {<Route path={$route} view=|| view! {<PathComponent path={PATHS[$route].clone()} /> } />}),+]
    };
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            {Routes(RoutesProps { base: None, children: Box::new(|| Fragment::new(routes!(
                "/",
                "/prince",
                "/prince/leave",
                "/prince/stay",
                "/prince/stay/ask-for-dog",
                "/prince/stay/no-dog",
                "/prince/stay/ask-for-dog/pet-dog",

                "/princess"
            ))) })}
        </Router>
    }
}
