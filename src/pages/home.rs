use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_use::{use_mutation_observer_with_options, UseMutationObserverOptions};

use crate::components::carousel::{Carousel, CarouselElement};
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::WheelEvent;

use crate::helpers::circular_buffer::CircularBuffer;
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

    let canvas_ref: NodeRef<html::Canvas> = NodeRef::new();
    let (canvas_loaded, set_canvas_loaded) = signal(false);

    use_mutation_observer_with_options(
        canvas_ref,
        move |mutations, _| {
            if let Some(mutation) = mutations.first() {
                if let Some(attribute_name) = mutation.attribute_name() {
                    if attribute_name == "tabindex" {
                        set_canvas_loaded.set(true);
                    }
                }
            }
        },
        UseMutationObserverOptions::default().attributes(true),
    );

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
            let _ = audio.play();
        }
        if circular_buffer.read().to_string() == "umaruuu" {
            set_easter_egg(true);
            let mut buffer = circular_buffer.read().clone();
            buffer.clear();
            set_circular_buffer(buffer);
            let audio = bad_apple_ref.get().unwrap();
            audio.set_src("/img/umaru.wav");
            audio.set_volume(0.1);
            let _ = audio.play();
        }
        if circular_buffer.read().to_string() == "ihatefun" {
            set_easter_egg(false);
            let mut buffer = circular_buffer.read().clone();
            buffer.clear();
            set_circular_buffer(buffer);
            let audio = bad_apple_ref.get().unwrap();
            let _ = audio.pause();
        }
        console_log(circular_buffer.read().to_string().as_str())
    });

    Effect::new(move |_| {
        initGame().unwrap();
    });

    on_cleanup(move || handle.remove());

    view! {
        <div>
            <Show
                when=move || { canvas_loaded.get() }
                fallback=|| {
                    view! {
                        <div
                            id="main"
                            class="grid absolute top-0 left-0 justify-center content-center w-screen h-screen z-2"
                        >
                            <div class="p-3 w-32 h-32 bg-gradient-to-bl from-pink-400 via-pink-500 to-pink-600 rounded-full animate-spin md:w-48 md:h-48 drop-shadow-2xl aspect-square">
                                <div class="w-full h-full rounded-full bg-[rgb(54,5,42)] background-blur-md"></div>
                            </div>
                            <div class="grid justify-center content-center">
                                <span class="text-pink-500">"Loading..."</span>
                            </div>
                        </div>
                    }
                }
            >

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
            </Show>
        </div>
        <canvas
            on:wheel=disable_wheel
            id="canvas"

            node_ref=canvas_ref
            class="fixed top-0 left-0 z-0 w-full h-screen outline-none bg-[rgb(54,5,42)]"
        />
    }
}
