mod text_input;

use gloo::console::log;
use reqwasm::http::Request;
use serde_json::json;
use text_input::{Props, TextInput};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

struct App {
    site_data: String,
}

enum Msg {
    Load,
    Unload,
    Query,
    SiteData(String),
}

impl From<String> for Msg {
    fn from(s: String) -> Self {
        Msg::SiteData(s)
    }
}

impl App {
    fn load(&self, ctx: &Context<Self>) {
        let load_endpoint = format!("http://54.202.82.203:8000/load");
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_load = Request::get(&load_endpoint).send().await;
            match fetched_load {
                Ok(response) => {
                    let str = response.text().await;
                    match str {
                        Ok(str) => {
                            link.send_message(Msg::from(str));
                        }
                        Err(err) => {}
                    }
                }
                Err(err) => {}
            }
        });
    }

    fn unload(&self, ctx: &Context<Self>) {
        let unload_endpoint = format!("http://54.202.82.203:8000/unload");
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_load = Request::get(&unload_endpoint).send().await;
            match fetched_load {
                Ok(response) => {
                    let str = response.text().await;
                    match str {
                        Ok(str) => {
                            link.send_message(Msg::from(str));
                        }
                        Err(err) => {}
                    }
                }
                Err(err) => {}
            }
        });
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            site_data: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Load => {
                self.load(ctx.clone());
            }
            Msg::Unload => {
                self.unload(ctx.clone());
            }
            Msg::Query => {}
            Msg::SiteData(data) => {
                self.site_data = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text_input_props = Props { name: "".into() };
        let onchange = Callback::from(|event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            log!(value);
        });
        html! {
        <div>
            <p class = "name-text"> {"First Name"} </p><p class = "name-text"> {"Last Name"} </p>
            <br/>
            <input type="text" class ="input" name={text_input_props.name.clone()} onchange = {onchange} />
            <input type="text" class ="input"  />
            <br/>
            <button class = "button" onclick={ctx.link().callback(|_| Msg::Load)}>{ "Load" }</button>
            <button class = "button" onclick={ctx.link().callback(|_| Msg::Unload)}>{ "Unload" }</button>
            <button class = "button" onclick={ctx.link().callback(|_| Msg::Query)}>{ "Query" }</button>
            <pre> {&self.site_data} </pre>
        </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
