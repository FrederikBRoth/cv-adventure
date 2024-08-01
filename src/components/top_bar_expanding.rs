use ev::SubmitEvent;
use leptos::*;
use leptos_dom::logging::console_log;
#[component]
pub fn TopBarExpanding(#[prop(into)] on_submit_form: Callback<String>) -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let (clicked, set_clicked) = create_signal(false);
    let input_element: NodeRef<html::Input> = create_node_ref();
    let submit = move |_| {
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        console_log("test");
        set_name(value);
        on_submit_form.call(name());
    };
    view! {
        <div
            class="transition ease-in-out w-full flex flex-col justify-center items-center h-14 absolute top-[-2.5rem] duration-300"
            style:transform=move || {
                if clicked() { "translateY(2.5rem)" } else { "translateY(0)" }
            }
        >
            <div class="w-full h-10 bg-teal-600 flex justify-center">
                <input class="m-2" type="text" value=name node_ref=input_element />
                <button class="bg-teal-100 m-2" on:click=submit>
                    "Change name"
                </button>
            </div>
            <div
                class="w-10 h-4 bg-teal-600 rounded-b"
                on:click=move |_| { set_clicked.update(|n| *n = !*n) }
            ></div>
        </div>
    }
}
