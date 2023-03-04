use yew::prelude::*;
use gloo::console::log;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name : String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let onchange = Callback::from(|event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(value);
    });
    html! {
        <input type="text" class ="input" name={props.name.clone()} onchange = {onchange} />
    }
}