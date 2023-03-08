mod text_input;

use gloo::console::log;
use text_input::{Props, TextInput};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use reqwasm::http::Request;

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
        let load_endpoint =
            format!("http://127.0.0.1:8000/load");
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_load = Request::get(&load_endpoint)
                .send()
                .await;
            match fetched_load {
                Ok(response) => {
                    let str = response.text().await;
                    match str {
                        Ok(str) => {
                            log!(&*str);
                        }
                        Err(err) => {
                        }
                    }
                }
                Err(err) => {
                }
            }
            //link.send_message(Msg::from(fetched_load));
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
            Msg::Unload => {}
            Msg::Query => {}
            Msg::SiteData(data) => {
                self.site_data = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text_input_props = Props { name: "".into() };
        html! {
        <div>
            <button class = "button" onclick={ctx.link().callback(|_| Msg::Load)}>{ "Load" }</button>
            <br/>
            <button class = "button" onclick={ctx.link().callback(|_| Msg::Unload)}>{ "Unload" }</button>
            <br/>
            <TextInput name={text_input_props.name} />
            <br/>
            <button class = "button" onclick={ctx.link().callback(|_| Msg::Query)}>{ "Query" }</button>
            <p> {&*self.site_data} </p>
        </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
