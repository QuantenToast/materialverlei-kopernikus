use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod error;
mod services;
mod types;
use components::material::*;

use types::material::Material;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
}

#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <>
                    <h1>{ "Home" }</h1>
                    <Link<Route> to={Route::Secure}>{ "click here to go to secure" }</Link<Route>>
                    </>
            }
        }
        Route::Secure => html! {
            <Secure />
        },
    }
}

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
                    let fetched_material: Result<Vec<Material>, error::Error> =
                        services::req::request_get(String::from("0")).await;

                    match fetched_material {
                        Ok(f) => material.set(f),
                        Err(e) => error.set(format!("error: {:#?}", e)),
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
                    <lable id="title">{"Materialverlei"}</lable>
                    <div id="ham">
                        <lable>{"≡"}</lable>
                    </div>
                </div>
                <div id="util-bar"/>
            </div>
            <div id="greeter"/>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
                <MaterialListComponent materialien={(*material).clone()}/>
            <div class="footer"/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
