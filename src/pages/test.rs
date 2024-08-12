use std::ops::Deref;

use ev::DragEvent;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, File, FileList, FileReader, HtmlInputElement, Url};

#[wasm_bindgen(module = "/public/gameimport.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn initGame() -> Result<JsValue, JsValue>;
}

#[component]
pub fn Game() -> impl IntoView {
    create_effect(move |_| {
        initGame();
    });

    view! { <div id="wasm-example" class="bg-sky-300"></div> }
}
