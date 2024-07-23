use crate::components::counter_btn::Button;
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

                <picture class="bg-current">
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1 class="m-0 auto">"Welcome to Leptos"</h1>
                <h2>"This is goated, pladwawds for reals! AGAIUNdawdawdaw awdawdd !"</h2>
                <h2>"this is oemHAHAHAHHAga goated"</h2>

                <p>This is a full pipeline test</p>
                <div class="flex justify-evenly">
                    <Button />
                    <Button increment=5 />
                </div>

            </div>
        </ErrorBoundary>
    }
}
