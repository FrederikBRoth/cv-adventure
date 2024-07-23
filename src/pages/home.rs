use crate::components::{counter_btn::Button, dynamic_video::DynamicVideo, top_bar::TopBar};
use leptos::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (url, set_url) =
        create_signal("https://www.youtube.com/embed/vg0Tmydj29M?si=E2cgbgxXHqNB0Ec1".to_string());
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

            <TopBar setter=set_url />

            <div class="flex flex-col justify-around items-center">

                <h1 class="m-0 auto font-sans test">"Welcome to the site"</h1>
                <h2>"This is my new website. Cool things to come!"</h2>
                <h2>"Really cool things!"</h2>

                <div class="flex justify-evenly">
                    <Button />
                    <Button increment=5 />
                </div>

                <DynamicVideo link=url />
                <picture class="bg-red-300 p-2 rounded-xl absolute bottom-2 left-2">
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="50"
                        width="100"
                    />
                </picture>

            </div>
        </ErrorBoundary>
    }
}
