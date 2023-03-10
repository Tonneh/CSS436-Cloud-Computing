use gloo::console::log;
use reqwasm::http::Request;
use std::ops::Add;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

struct App {
    site_data: String,
    last_name: String,
    first_name: String,
}

enum Msg {
    Load,
    Unload,
    Query,
    SiteData(String),
    LastNameChange(String),
    FirstNameChange(String),
}

impl From<String> for Msg {
    fn from(s: String) -> Self {
        Msg::SiteData(s)
    }
}

impl App {
    fn load(&self, ctx: &Context<Self>) {
        let load_endpoint = format!("http://35.87.106.62:8000/load");
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_load = Request::get(&load_endpoint).send().await;
            match fetched_load {
                Ok(response) => {
                    let str = response.text().await;
                    match str {
                        Ok(str) => {
                            link.send_message(Msg::from("Loaded: \n".to_string() + &*str));
                        }
                        Err(err) => {
                            link.send_message(Msg::from("Error Loading".to_string()));
                        }
                    }
                }
                Err(err) => {
                    link.send_message(Msg::from("Error Loading".to_string()));
                }
            }
        });
    }

    fn unload(&self, ctx: &Context<Self>) {
        let unload_endpoint = format!("http://35.87.106.62:8000/unload");
        let link = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_unload = Request::get(&unload_endpoint).send().await;
            match fetched_unload {
                Ok(response) => {
                    let str = response.text().await;
                    match str {
                        Ok(str) => {
                            link.send_message(Msg::from(str));
                        }
                        Err(err) => {
                            link.send_message(Msg::from("Error Unloading".to_string()));
                        }
                    }
                }
                Err(err) => {
                    link.send_message(Msg::from("Error Unloading".to_string()));
                }
            }
        });
    }

    fn query(&self, ctx: &Context<Self>) {
        let mut query_endpoint = "http://35.87.106.62:8000/query".to_string();
        let last_name = self.last_name.clone();
        let first_name = self.first_name.clone();
        let link = ctx.link().clone();
        if last_name.is_empty() && first_name.is_empty() {
            link.send_message(Msg::from("Empty first and last name".to_string()));
            return;
        }
        if !last_name.is_empty() && !first_name.is_empty() {
            query_endpoint = format!("{}/full/{}%20{}", query_endpoint, last_name, first_name);
        } else if !last_name.is_empty() {
            query_endpoint = format!("{}/last/{}", query_endpoint, last_name);
        } else {
            query_endpoint = format!("{}/first/{}", query_endpoint, first_name);
        }
        let mod_query_endpoint = query_endpoint;
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_query = Request::get(&mod_query_endpoint).send().await;
            match fetched_query {
                Ok(response) => {
                    let str = response.text().await;
                    match str {
                        Ok(str) => {
                            link.send_message(Msg::from("Queried: \n".to_string() + &*str));
                        }
                        Err(err) => {
                            link.send_message(Msg::from("Error querying".to_string()));
                        }
                    }
                }
                Err(err) => {
                    link.send_message(Msg::from("Error querying".to_string()));
                }
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
            last_name: String::new(),
            first_name: String::new(),
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
            Msg::Query => {
                self.query(ctx.clone());
            }
            Msg::SiteData(data) => {
                self.site_data = data;
            }
            Msg::LastNameChange(data) => {
                self.last_name = data;
            }
            Msg::FirstNameChange(data) => {
                self.first_name = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <p class = "name-text"> {"First Name"} </p><p class = "name-text"> {"Last Name"} </p>
            <br/>
            <input type="text" class="input" oninput={ctx.link().callback(|e: web_sys::InputEvent| {
                let value = e.target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                    Msg::FirstNameChange(value)
            })} />
            <input type="text" class="input" oninput={ctx.link().callback(|e: web_sys::InputEvent| {
                let value = e.target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                    Msg::LastNameChange(value)
            })} />
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
