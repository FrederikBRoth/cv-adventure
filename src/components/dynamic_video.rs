use std::string;

use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
#[component]
pub fn DynamicVideo(link: ReadSignal<String>) -> impl IntoView {
    view! {
        <div>
            <iframe
                width="560"
                height="315"
                src=link.clone()
                title="YouTube video player"
                allow="autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                referrerpolicy="strict-origin-when-cross-origin"
            ></iframe>
        </div>
        <span>{link}</span>
    }
}
