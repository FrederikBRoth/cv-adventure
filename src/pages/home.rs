use crate::components::{
    counter_btn::Button, dynamic_video::DynamicVideo, top_bar::TopBar, video_player::VideoPlayer,
};
use leptos::prelude::*;
use leptos::*;
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>


            <div class="flex flex-col justify-around items-center">

                <h1 class="m-0 auto font-sans test">"Welcome to the site"</h1>
                <h2>"This is my new website. Cool things to come!"</h2>
                <h2>"Really cool things! yahooo"</h2>


            </div>
        </ErrorBoundary>
    }
}
