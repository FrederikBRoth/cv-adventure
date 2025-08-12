use leptos::prelude::*;
/// A parameterized incrementing button
#[component]
pub fn Carousel(children: ChildrenFragment) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <div class="pr-1 pl-1 min-w-full max-w-full h-full">{child}</div> })
        .collect::<Vec<_>>();

    let children_size = children.len();
    let (index, set_index) = signal(0);
    view! {
        <div class="flex overflow-hidden relative flex-row w-full h-full">
            <Show when=move || { index.get() != 0 }>
                <div class="absolute left-0 h-full z-2">
                    <div
                        class="flex items-center pl-2 h-full rounded-tl-lg rounded-bl-lg transition duration-100 ease-in-out hover:bg-gradient-to-r hover:from-black/25 hover:to-black/0"
                        on:click=move |_| set_index(
                            (index.get() + children_size - 1) % children_size,
                        )
                    >
                        <button type="button">

                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <polyline points="15 18 9 12 15 6"></polyline>
                            </svg>
                        </button>
                    </div>
                </div>

            </Show>
            <div class="flex overflow-hidden relative flex-row pt-2 pr-1 pb-2 pl-1 w-full h-full">
                <div
                    class="flex flex-row w-full h-full rounded-lg"
                    style=move || {
                        format!(
                            "transform: translateX(-{}%); transition: transform 0.5s ease;",
                            index.get() * 100,
                        )
                    }
                >
                    {children}
                </div>

            </div>
            <Show when=move || { index.get() != children_size - 1 }>
                <div class="absolute right-0 h-full z-2">
                    <div
                        class="flex items-center pr-2 h-full rounded-tr-lg rounded-br-lg transition duration-100 ease-in-out hover:bg-gradient-to-l hover:from-black/25 hover:to-black/0"
                        on:click=move |_| set_index((index.get() + 1) % children_size)
                    >

                        <button type="button">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <polyline points="9 18 15 12 9 6"></polyline>
                            </svg>

                        </button>
                    </div>
                </div>

            </Show>

        </div>
    }
}

#[component]
pub fn CarouselElement(children: Children) -> impl IntoView {
    view! { <div class="p-2 h-full bg-pink-300 rounded-lg">{children()}</div> }
}
