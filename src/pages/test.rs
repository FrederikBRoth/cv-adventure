use std::ops::Deref;

use ev::DragEvent;
use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys, File, FileList, FileReader, HtmlInputElement, Url};

#[wasm_bindgen(module = "/public/gameimport.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn initGame() -> Result<JsValue, JsValue>;
}

#[component]
pub fn Game() -> impl IntoView {
    Effect::new(move |_| {
        initGame();
    });

    view! { <div id="wasm-example" class="bg-sky-300"><canvas id="canvas" class="bg-sky-300"></canvas>Wow alright!</div> }
}
