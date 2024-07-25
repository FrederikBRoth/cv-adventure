use std::ops::Deref;

use leptos::*;
use leptos_dom::logging::console_log;
use web_sys::js_sys;
use web_sys::DragEvent;
use web_sys::Url;

#[component]
pub fn VideoPlayer() -> impl IntoView {
    let (videourl, set_videourl) = create_signal("".to_string());
    pub fn file_dropped(ev: DragEvent) -> String {
        // Resize the window to 500px by 500px.
        ev.prevent_default();
        match ev.data_transfer() {
            Some(data_transfer) => {
                let files: Vec<web_sys::File> = data_transfer
                    .files()
                    .map(|f| js_sys::Array::from(&f).to_vec())
                    .unwrap_or_default()
                    .into_iter()
                    .map(web_sys::File::from)
                    .collect();
                let file = files.first().unwrap();
                Url::create_object_url_with_blob(file.deref()).unwrap()
            }
            _ => "".to_string(),
        }
    }

    pub fn file_dragged(ev: DragEvent) {
        ev.prevent_default();
        console_log("asawd");
    }
    view! {
        <div
            class="w-3/6 h-96 bg-red-300 flex justify-center items-center"
            on:drop=move |ev: DragEvent| { set_videourl.update(|url| *url = file_dropped(ev)) }
            on:dragover=move |ev: DragEvent| { file_dragged(ev) }
        >
            "Drop file here!"
            <video class="w-full h-full" src=videourl controls autoplay></video>
        </div>
    }
}
