use components::counter_btn::{Button, ButtonPropsBuilder_Error_Repeated_field_increment};
use html::Button;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::birthday::Birthday;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
// use crate::pages::test::VideoTranscoder;
/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Dank website" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/birthday" view=Birthday />
                // <Route path="/test" view=VideoTranscoder />
                <Route path="/*" view=NotFound />
            </Routes>
        </Router>
    }
}
