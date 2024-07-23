use std::string;

use leptos::*;

#[component]
pub fn DynamicVideo(link: ReadSignal<String>) -> impl IntoView {
    view! {
        <div>
            <iframe
                width="560"
                height="315"
                src=link.clone()
                title="YouTube video player"
                frameborder="0"
                allow="autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                play=true
                referrerpolicy="strict-origin-when-cross-origin"
            ></iframe>
        </div>
        <span>{link}</span>
    }
}
