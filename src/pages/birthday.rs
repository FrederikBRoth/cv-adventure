use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_dom::logging::console_log;
use web_sys::window;
use ev::{DragEvent, SubmitEvent};
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, File, FileList, FileReader, HtmlInputElement, Url};
use crate::components::top_bar_expanding::TopBarExpanding;

#[wasm_bindgen(module = "/public/ffmpegSetup.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn transcodeVideo(inputName: JsValue) -> Result<JsValue, JsValue>;
}


#[component]
pub fn Birthday() -> impl IntoView {
    let (videourl, set_videourl) = create_signal("/img/basesong.mp3".to_string());
    let (loading, set_loading) = create_signal(false);
    fn on_submit(name: String, videourl: WriteSignal<String>, loading: WriteSignal<bool> ) {
        // stop the page from reloading!
        loading.update(|n| *n = true);

        // here, we'll extract the value from the input
       spawn_local(async move {
            match transcodeVideo(JsValue::from(name)).await {
                Ok(js_blob) => {
                    let url = web_sys::Url::create_object_url_with_blob(&js_blob.unchecked_into()).unwrap();
                    videourl.update(|value| { *value = url; });
                    loading.update(|n| {*n = false});
                }
                Err(e) => {
                    let err = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
                    web_sys::console::log_1(&err.into()); // Log the error to the console
                }
            }
        });

    };
    

    pub fn file_dragged(ev: DragEvent) {
        ev.prevent_default();
    }
    view! {
        <div class="w-[100vw] h-[100vh] bg-birthday-image bg-cover flex justify-center items-center">
            <TopBarExpanding on_submit_form=move |name: String| {
                on_submit(name, set_videourl, set_loading)
            } />
            // on_submit defined below
            <audio id="audio" src=videourl loop autoplay></audio>
            <Show when=move || { loading() }>
                <div class="w-2/6 h-2/6 bg-teal-600 opacity-90 rounded flex flex-col justify-center items-center">
                    <img class="w-3/4 h-3/4 animate-pulse" src="/img/jerma-rat.webp" />
                    "Loading"
                </div>
            </Show>
        </div>
    }
}
