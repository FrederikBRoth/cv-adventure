use ev::MouseEvent;
use leptos::prelude::*;
use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn Carousel(children: ChildrenFragment) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! {<div class="max-w-full min-w-full h-full p-1">{child}</div>})
        .collect::<Vec<_>>();

    let children_size = children.len();
    let (index, set_index) = signal(0);
    view! {
        <div class="relative w-full h-full overflow-hidden">
            <button class="z-2 absolute top-1/2 left-0" type="button" on:click=move |_| set_index((index.get()+children_size-1) % children_size)>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="15 18 9 12 15 6"></polyline>
                </svg>
            </button>
            <div class="flex flex-row w-full h-full rounded-lg" style=move || {
                format!(
                "transform: translateX(-{}%); transition: transform 0.5s ease;",
                index.get() * 100
            )}>
                {children}
            </div>
            <button class="z-2 absolute top-1/2 right-0" type="button" on:click=move |_| set_index((index.get()+1) % children_size)>
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"></polyline>
                </svg>
            </button>
        </div>
    }
}

#[component]
pub fn CarouselElement(children: Children) -> impl IntoView {
    view! {
    <div class="h-full rounded-lg p-2 bg-pink-300">
     {children()}
    </div>
    }
}

