use ev::MouseEvent;
use leptos::*;

/// A parameterized incrementing button
#[component]
pub fn VideoButton(name: String, #[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <button class="bg-red-300 hover:bg-red-700 p-2 m-2" on:click=on_click>
            {name}
        </button>
    }
}
