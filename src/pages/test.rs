use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/gameimport.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn initGame() -> Result<JsValue, JsValue>;
}

#[component]
pub fn Game() -> impl IntoView {
    Effect::new(move |_| {
        initGame().unwrap();
    });

    view! {
        <div id="wasm-example" class="bg-sky-300">
            <canvas id="canvas" class="bg-sky-300"></canvas>
            Wow alright!
        </div>
    }
}
