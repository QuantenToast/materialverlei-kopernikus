use yew::prelude::*;

mod components;
use components::material::*;

use reqwasm::http::Request;

#[function_component(App)]
fn app_component() -> Html {
    let material = use_state(|| Vec::new());
    let error = use_state(|| String::new());
    {
        let material = material.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |_| {
                let material = material.clone();
                let error = error.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("http://81.169.248.14/api/{page}", page = 0);
                    let fetched_material = Request::get(&url).send().await;

                    match fetched_material {
                        Ok(response) => match response.json().await {
                            Ok(f) => material.set(f),
                            Err(e) => error.set(format!("parser: {e}")),
                        },
                        Err(e) => error.set(format!("server: {e}")),
                    };
                });
                || ()
            },
            (),
        )
    }

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
            <MaterialListComponent materialien={(*material).clone()}/>
            <div class="footer"/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
