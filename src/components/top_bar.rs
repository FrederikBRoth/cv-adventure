use crate::components::video_button::VideoButton;
use leptos::*;
#[component]
pub fn TopBar(setter: WriteSignal<String>) -> impl IntoView {
    // let styler_class = style!();
    view! {
        <div class="flex justify-around bg-red-700 ssawadd ">
            <VideoButton
                name="Dagoth".to_string()
                on_click=move |_| {
                    setter
                        .update(|value| {
                            *value = "https://www.youtube.com/embed/iR-K2rUP86M?si=-o9MXOLqNbap58Af?autoplay=1&controls=0 "
                                .to_string();
                        })
                }
            />
            <VideoButton
                name="Lykkehjulet".to_string()
                on_click=move |_| {
                    setter
                        .update(|value| {
                            *value = "https://www.youtube.com/embed/vcSJ3GydHqM?si=-o9MXOLqNbap58Af?autoplay=1&controls=0 "
                                .to_string();
                        })
                }
            />
            <VideoButton
                name="DK Rap".to_string()
                on_click=move |_| {
                    setter
                        .update(|value| {
                            *value = "https://www.youtube.com/embed/9FoTgirSjkc?si=-09MXOLqNbap58Af?autoplay=1&controls=0 "
                                .to_string();
                        })
                }
            />
        </div>
    }
}
