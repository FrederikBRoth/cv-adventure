use ev::SubmitEvent;
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_dom::logging::console_log;
use web_sys::MouseEvent;
#[component]
pub fn TopBarExpanding(mut on_submit_form: impl FnMut(String)  + 'static ) -> impl IntoView {
    let (name, set_name) = signal("Uncontrolled".to_string());
    let (clicked, set_clicked) = signal(false);

    view! {
        <div
            class="transition ease-in-out w-full flex flex-col justify-center items-center h-14 absolute top-[-2.5rem] duration-300"
            style:transform=move || {
                if clicked() { "translateY(2.5rem)" } else { "translateY(0)" }
            }
        >
            <div class="w-full h-10 bg-teal-600 flex justify-center">
                <input class="m-2" type="text" bind:value=(name, set_name) />
                <button class="bg-teal-100 m-2" on:click=move |ev: MouseEvent| {
                            on_submit_form(name.get())
                    }>
                    "Change name"
                </button>
            </div>
            <div
                class="w-10 h-4 bg-teal-600 rounded-b"
                on:click=move |_| { set_clicked.update(|n| *n = !*n) }
            ></div>
        </div>
    }
}
