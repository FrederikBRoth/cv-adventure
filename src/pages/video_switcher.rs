use leptos::prelude::*;

use crate::components::{dynamic_video::DynamicVideo, top_bar::TopBar, video_player::VideoPlayer};

#[component]
pub fn VideoSwitcher() -> impl IntoView {
    let (url, set_url) =
        signal("https://www.youtube.com/embed/vg0tmydj29m?si=e2cgbgxxhqnb0ec1".to_string());

    view! {
        <TopBar setter=set_url />

            <div class="flex flex-col justify-around items-center">

                <h1 class="m-0 auto font-sans test">"Welcome to the site"</h1>
                <h2>"This is my new website. Cool things to come!"</h2>
                <h2>"Really cool things! yahooo"</h2>

                <DynamicVideo link=url />
                <picture class="bg-red-300 p-2 rounded-xl absolute bottom-2 left-2">
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                </picture>
                <VideoPlayer />

            </div>

    }
}
