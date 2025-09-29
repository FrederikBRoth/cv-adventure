use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;

use crate::components::carousel::{Carousel, CarouselElement};
use leptos::*;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsValue;
use web_sys::WheelEvent;

use crate::helpers::circular_buffer::{self, CircularBuffer};
#[wasm_bindgen(module = "/public/gameimport.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn initGame() -> Result<JsValue, JsValue>;
}

#[component]

pub fn Home() -> impl IntoView {
    let buffer: CircularBuffer<String> = CircularBuffer::new(8);
    let (circular_buffer, set_circular_buffer) = signal(buffer);
    let (easter_egg, set_easter_egg) = signal(false);
    let bad_apple_ref: NodeRef<html::Audio> = NodeRef::new();

    let disable_wheel = move |ev: WheelEvent| {
        let scroll = window().scroll_y().unwrap().clone().to_string();
        ev.stop_immediate_propagation();
        console_log(scroll.as_str());
    };

    let handle = window_event_listener(ev::keypress, move |ev| {
        let mut buffer = circular_buffer.read().clone();
        buffer.insert(ev.key());

        set_circular_buffer(buffer);
    });

    Effect::new(move |_| {
        if circular_buffer.read().to_string() == "badapple" {
            set_easter_egg(true);
            let mut buffer = circular_buffer.read().clone();
            buffer.clear();
            set_circular_buffer(buffer);
            let audio = bad_apple_ref.get().unwrap();
            audio.set_src("/img/badapple.wav");
            audio.set_volume(0.1);
            audio.play();
        }
        if circular_buffer.read().to_string() == "umaruuu" {
            set_easter_egg(true);
            let mut buffer = circular_buffer.read().clone();
            buffer.clear();
            set_circular_buffer(buffer);
            let audio = bad_apple_ref.get().unwrap();
            audio.set_src("/img/umaru.wav");
            audio.set_volume(0.1);
            audio.play();
        }
        if circular_buffer.read().to_string() == "ihatefun" {
            set_easter_egg(false);
            let mut buffer = circular_buffer.read().clone();
            buffer.clear();
            set_circular_buffer(buffer);
            let audio = bad_apple_ref.get().unwrap();
            audio.pause();
        }
        console_log(circular_buffer.read().to_string().as_str())
    });

    Effect::new(move |_| {
        initGame().unwrap();
    });

    on_cleanup(move || handle.remove());

    view! {
        <audio node_ref=bad_apple_ref></audio>

        <div id="main" class="grid grid-cols-12 w-full grid-rows-300 h-2000">

            <Show
                when=move || { !easter_egg.get() }
                fallback=|| {
                    view! {
                        <div class="col-start-1 col-end-13 row-start-1 row-end-2 bg-pink-600 z-2">
                            "Click here and type 'ihatefun' on your keyboard to stop the fun"
                        </div>
                    }
                }
            >
                // CSharp info
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-end-9 z-2 row-start-38 row-end-62">
                    <Carousel>
                        <CarouselElement>
                            <div>
                                <h2 class="font-bold">"Student job at PFA"</h2>
                                <p class="pt-2">
                                    "Through my Master studies, i worked at PFA where i had the pleasure of working on a myriad of different projects throughout my almost 3 year tenure in the organization"
                                </p>
                                <p class="pt-2">
                                    "Primarily developed and maintained an internal application to monitor and manage inter-system communication across the organization"
                                </p>
                                <ul class="pt-2 pl-5 list-disc list-outside">
                                    <li>
                                        "Robust integration of EF Core, ensuring safe and fast database queries when serving full context search"
                                    </li>
                                    <li>
                                        "Implemented robust generic solutions through reflection, in order to fascilitate different views and filters based on asset type"
                                    </li>
                                    <li>
                                        "Designed and implemented clean, reusable Blazor components to minimize code duplication and streamline the development process"
                                    </li>
                                    <li>
                                        "Utilized Attributes in order to specify input form types for easy modification of form layout"
                                    </li>
                                </ul>
                            </div>
                        </CarouselElement>
                        <CarouselElement>
                            <div>
                                <h2 class="font-bold">"Student job at PFA"</h2>
                                <p class="pt-2">
                                    "Through my Master studies, i worked at PFA where i had the pleasure of working on a myriad of different projects throughout my almost 3 year tenure in the organization"
                                </p>
                                <p class="pt-2">
                                    "Primarily developed and maintained an internal application to monitor and manage inter-system communication across the organization"
                                </p>
                                <ul class="pt-2 pl-5 list-disc list-outside">
                                    <li>
                                        "Robust integration of EF Core, ensuring safe and fast database queries when serving full context search"
                                    </li>
                                    <li>
                                        "Implemented robust generic solutions through reflection, in order to fascilitate different views and filters based on asset type"
                                    </li>
                                    <li>
                                        "Designed and implemented clean, reusable Blazor components to minimize code duplication and streamline the development process"
                                    </li>
                                    <li>
                                        "Utilized Attributes in order to specify input form types for easy modification of form layout"
                                    </li>
                                </ul>
                            </div>
                        </CarouselElement>
                        <CarouselElement>Yet another thing here!</CarouselElement>
                    </Carousel>
                </div>

                // Rust info
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-start-5 z-2 row-start-70 row-end-94"></div>

                // C++ info
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-end-9 z-2 row-start-102 row-end-126"></div>

                // Containerization/Devops info
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-start-5 z-2 row-start-134 row-end-159"></div>

                // Whatever
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-end-5 z-2 row-start-167 row-end-179 md:row-end-192"></div>

                // Whatever 2
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-start-9 z-2 row-start-180 row-end-192 md:row-start-167"></div>

                // whatever 3
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-end-9 z-2 row-start-200 row-end-225"></div>

                // whatevr 4
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-start-5 z-2 row-start-233 row-end-258"></div>
                // Whatever
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-end-5 z-2 row-start-266 row-end-278 md:row-end-291"></div>

                // Whatever 2
                <div class="col-start-2 col-end-12 bg-pink-500 rounded-xl opacity-75 md:col-start-9 z-2 row-start-279 row-end-291 md:row-start-266"></div>

                <div class="flex flex-col col-start-3 col-end-11 row-start-4 justify-around items-center bg-pink-500 rounded-xl z-2 row-end-10">
                    <h1 class="m-0 font-sans auto test">"Welcome to the site"</h1>
                    <h2>"This is my new website. Cool things to come! Right?"</h2>
                    <h2>"Really cool things! yahooo"</h2>
                    <h2>
                        "Press SPACE to stop animation. When animation is stopped, you can use WASD to spin around the object and zoom in and out"
                    </h2>
                    <h2>"You can also RMB to do cool explosion. Wow"</h2>
                </div>

                <div class="col-start-1 col-end-13 row-start-1 row-end-2 bg-pink-600 z-2"></div>
            </Show>

        </div>
        <canvas
            on:wheel=disable_wheel
            id="canvas"
            class="fixed top-0 left-0 z-0 w-full h-screen outline-none bg-[rgb(54,5,42)]"
        />
    }
}
