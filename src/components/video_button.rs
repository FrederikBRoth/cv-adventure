use ev::MouseEvent;
use leptos::prelude::*;
use leptos::*;
/// A parameterized incrementing button
#[component]
pub fn VideoButton(name: String, on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button class="bg-red-400 hover:bg-red-700 p-2 m-2 rounded"
                on:click=on_click>
            {name}
        </button>
    }
}
