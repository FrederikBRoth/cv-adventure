use leptos::wasm_bindgen::JsCast;
use leptos::*;
use web_sys::window;
#[component]
pub fn Birthday() -> impl IntoView {
    pub fn load_audio() {
        let audio = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("audio")
            .unwrap()
            .dyn_into::<web_sys::HtmlAudioElement>()
            .unwrap();
        audio.load();
        audio.play();
    }
    view! {
        <div class="w-[100vw] h-[100vh] bg-birthday-image bg-cover" on:click=|_| { load_audio() }>
            <audio id="audio" src="/img/basesong.mp3" loop></audio>
        </div>
    }
}
