use std::{ ops::Deref};

use ev::DragEvent;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, File, FileList, FileReader, HtmlInputElement, Url};

#[wasm_bindgen(module = "/src/ffmpeg/ffmpegSetup.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn transcodeVideo(inputFile: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn VideoTranscoder() -> impl IntoView {
    let (videourl, set_videourl) = create_signal("".to_string());
   
    pub fn file_dropped(ev: DragEvent, signal: WriteSignal<String>) {
    // Prevent the default browser behavior for the drop event
        ev.prevent_default();
    
        match ev.data_transfer() {
            Some(data_transfer) => {
                let files: Vec<File> = data_transfer
                    .files()
                    .map(|f| js_sys::Array::from(&f).to_vec())
                    .unwrap_or_default()
                    .into_iter()
                    .map(web_sys::File::from)
                    .collect();
                    
                if let Some(file) = files.first() {
                    // Clone the file for use in the async block
                    let file_clone = file.clone();
                    
                    spawn_local(async move {
                        match transcodeVideo(JsValue::from(file_clone)).await {
                            Ok(js_blob) => {
                                let url = web_sys::Url::create_object_url_with_blob(&js_blob.unchecked_into()).unwrap();
                                signal.update(|value| { *value = url; });
                            }
                            Err(e) => {
                                let err = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
                                web_sys::console::log_1(&err.into()); // Log the error to the console
                            }
                        }
                    });
                }
            },
            None => {
                // Handle the case where there is no data transfer
                web_sys::console::log_1(&"No data transfer available".into());
            }
        }
    }

    pub fn file_dragged(ev: DragEvent) {
        ev.prevent_default();
    }
    

    view! {
        <div
            class="w-3/6 h-96 bg-red-300 flex justify-center items-center"
            on:drop=move |ev: DragEvent| { file_dropped(ev, set_videourl) }
            on:dragover=move |ev: DragEvent| { file_dragged(ev) }
        >
            "Drop file here!"
        </div>
        <audio class="w-full h-full" src=videourl controls autoplay></audio>
    }
    
}
