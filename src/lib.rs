extern crate serde;
extern crate serde_derive;
extern crate yew;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

use serde_derive::Deserialize;

struct Materialverlei {
    link: ComponentLink<Self>,
    materialien: Option<Vec<Material>>,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Material>, anyhow::Error>),
}

impl Component for Materialverlei {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            materialien: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="main-container">
                <div class=classes!("navbar")>
                    <div id="top-bar">
                        <lable id="title">{"Materialverlei Kopernikus"}</lable>
                        <dropdown id="ham">
                            <lable>{"≡"}</lable>
                            <dropdown-element></dropdown-element>
                        </dropdown>
                    </div>
                    <div id="util-bar"></div>
                </div>
                <div id="greeter"></div>
                <div class=classes!("footer")></div>
            </div>
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Material {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub num_available: u32,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Materialverlei>::new().mount_to_body();
}