use leptos::wasm_bindgen::JsCast;
use leptos::*;
use leptos_dom::logging::console_log;
use web_sys::window;
use ev::{DragEvent, SubmitEvent};
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, File, FileList, FileReader, HtmlInputElement, Url};

#[wasm_bindgen(module = "/dist/img/ffmpegSetup.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn transcodeVideo(inputName: JsValue) -> Result<JsValue, JsValue>;
}


#[component]
pub fn Birthday() -> impl IntoView {
    let (videourl, set_videourl) = create_signal("/img/basesong.mp3".to_string());
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let (loading, set_loading) = create_signal(false);

    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent, signal: WriteSignal<String>| {
        // stop the page from reloading!
        set_loading(true);
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
        spawn_local(async move {
            match transcodeVideo(JsValue::from(name.get())).await {
                Ok(js_blob) => {
                    let url = web_sys::Url::create_object_url_with_blob(&js_blob.unchecked_into()).unwrap();
                    signal.update(|value| { *value = url; });
                    set_loading(false)
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
        <div class="w-[100vw] h-[100vh] bg-birthday-image bg-cover">
            // on_submit defined below
            <form on:submit=move |ev: SubmitEvent| { on_submit(ev, set_videourl) }>
                <input type="text" value=name node_ref=input_element />
                <input type="submit" value="Submit" />
            </form>
            <audio id="audio" src=videourl loop autoplay></audio>
            <Show when=move || { loading() }>
                <div class="w-full h-4/6 m-50 p-50 bg-red-300 flex justify-center items-center">
                    "Loading"
                </div>
            </Show>
        </div>
    }
}
