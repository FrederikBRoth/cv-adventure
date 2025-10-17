use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/mobiusimport.js")]
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
            <canvas
                id="canvas"

                class="fixed top-0 left-0 z-0 w-full h-screen outline-none bg-[rgb(54,5,42)]"
            />
        </div>
    }
}
