use leptos::ev::wheel;
use leptos::html::Canvas;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;
use log::Record;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, Event, KeyboardEvent, MouseEvent, WheelEvent};

#[wasm_bindgen(module = "/public/gameimport.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn initGame() -> Result<JsValue, JsValue>;
}

#[component]
pub fn Home() -> impl IntoView {
    let disable_wheel = move |ev: WheelEvent| {
        let scroll = window().scroll_y().unwrap().clone().to_string();
        ev.stop_immediate_propagation();
        console_log(scroll.as_str());
    };
    Effect::new(move |_| {
        println!("awd");
        initGame().unwrap();
    });

    view! {

            <div id="main" class="grid grid-cols-12 grid-rows-150 w-full h-1000">

            <div class="z-2 row-start-38 row-end-62 col-start-2 col-end-9 bg-pink-500 opacity-75 rounded-xl">

                </div>

                <div class="z-2 row-start-70 row-end-94 col-start-5 col-end-12 bg-pink-500 opacity-75 rounded-xl">


                </div>

                <div class="z-2 row-start-102 row-end-126 col-start-2 col-end-9 bg-pink-500 opacity-75 rounded-xl">

                </div>

                <div class="z-2 row-start-134 row-end-151 col-start-5 col-end-12 bg-pink-500 opacity-75 rounded-xl">


                </div>
                <div class="z-2 col-start-1 col-end-13 row-start-1 row-end-2 bg-pink-600">


                </div>
                <div class="z-2 flex flex-col justify-around items-center col-start-3 col-end-11 row-start-4 row-end-10 bg-pink-500 rounded-xl">
                    <h1 class="m-0 auto font-sans test">"Welcome to the site"</h1>
                    <h2>"This is my new website. Cool things to come!"</h2>
                    <h2>"Really cool things! yahooo"</h2>
                    <h2>"Press SPACE to stop animation. When animation is stopped, you can use WASD to spin around the object and zoom in and out"</h2>
                <h2>"You can also RMB to do cool explosion. Wow"</h2>
                </div>
            </div>
            <canvas on:wheel=disable_wheel id="canvas" class="fixed top-0 left-0 w-full h-screen bg-[rgb(54,5,42)] z-0 outline-none"/>


    }
}
