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
        <Title text="Possibly Doomed Prince"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            {Routes(RoutesProps { base: None, children: Box::new(|| Fragment::new(routes!(
                "/end",
                "/",
                "/prince",
                "/prince/leave",
                "/prince/stay",
                "/prince/stay/no-dog",
                "/prince/stay/no-dog/lie",
                "/prince/stay/no-dog/lie/wait-until-healed",
                "/prince/stay/no-dog/lie/leap",
                "/prince/stay/no-dog/lie/wait-until-healed/leap",
                "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog",
                "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog/help-crocodile",
                "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog/avoid-crocodile",
                "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog",
                "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog/help-crocodile",
                "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog/avoid-crocodile",
                "/prince/stay/no-dog/truth",
                "/prince/stay/no-dog/truth/accept-offer",
                "/prince/stay/no-dog/truth/accept-offer/ask-kill-dog",
                "/prince/stay/no-dog/truth/accept-offer/keep-dog",
                "/prince/stay/no-dog/truth/accept-offer/dog-kept",
                "/prince/stay/no-dog/truth/accept-offer/dog-kept/help-crocodile",
                "/prince/stay/no-dog/truth/accept-offer/dog-kept/avoid-crocodile",
                "/prince/stay/no-dog/truth/decline-offer",
                "/prince/stay/no-dog/truth/decline-offer/ask-kill-dog",
                "/prince/stay/no-dog/truth/decline-offer/ask-kill-dog/help-crocodile",
                "/prince/stay/no-dog/truth/decline-offer/ask-kill-dog/avoid-crocodile",
                "/prince/stay/no-dog/truth/decline-offer/keep-dog",
                "/prince/stay/no-dog/truth/decline-offer/keep-dog/help-crocodile",
                "/prince/stay/no-dog/truth/decline-offer/keep-dog/avoid-crocodile",
                "/prince/stay/ask-for-dog",
                "/prince/stay/ask-for-dog/pet-dog",
                "/prince/stay/ask-for-dog/pet-dog/lie",
                "/prince/stay/ask-for-dog/pet-dog/lie/leap",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog/help-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog/avoid-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog/help-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog/avoid-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/truth",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog/help-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog/avoid-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog/help-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog/avoid-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog",
                "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/leap",
                "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed",
                "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap",
                "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap/help-crocodile",
                "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap/avoid-crocodile",

                "/princess"
            ))) })}
        </Router>
    }
}
