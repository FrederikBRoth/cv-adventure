use components::counter_btn::{Button, ButtonPropsBuilder_Error_Repeated_field_increment};
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::birthday::Birthday;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::test::Game;
// use crate::pages::test::VideoTranscoder;
/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // sets the document title
        <Title text="Dank website" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("/") view=Home />
                <Route path=StaticSegment("/birthday") view=Birthday />
                <Route path=StaticSegment("/test") view=Game />
                <Route path=StaticSegment("/*") view=NotFound />
            </Routes>
        </Router>
    }
}
