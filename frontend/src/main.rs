extern crate reqwasm;
extern crate serde;
extern crate serde_derive;
extern crate yew;

use yew::prelude::*;

mod components;
use components::material::*;

use reqwasm::http::Request;

#[function_component(App)]
fn app_component() -> Html {
    let material = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let retry = {
        let material = material.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let material = material.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://81.169.248.14/api/{page}", page = 0);
                let fetched_material = Request::get(&url).send().await;

                match fetched_material {
                    Ok(response) => {
                        let json: Result<Vec<Material>, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                material.set(Some(f));
                            }
                            Err(e) => error.set(Some(format!("parser: {e}"))),
                        }
                    }
                    Err(e) => error.set(Some(format!("server: {e}"))),
                }
            });
        })
    };

    html! {
        <div id="main-container">
            <div class="navbar">
                <div id="top-bar">
                    <lable id="title">{"Materialverlei Kopernikus"}</lable>
                    <div id="ham">
                        <lable>{"≡"}</lable>
                    </div>
                </div>
                <div id="util-bar"/>
            </div>
            <div id="greeter"/>
            {
                match (*material).as_ref() {
                    Some(m) => html! {<MaterialListComponent materialien={m.clone()}/>},
                    None => match (*error).as_ref() {
                        Some(e) => {
                            html! {
                                <div>
                                    {"error"} {e}
                                    <button onclick={retry}>{"retry"}</button>
                                </div>
                            }
                        }
                        None => {
                            html! {
                                <div>
                                    {"No data yet"}
                                    <button onclick={retry}>{"Call API"}</button>
                                </div>
                            }
                        }
                    },
                }
            }
            <div class="footer"/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
