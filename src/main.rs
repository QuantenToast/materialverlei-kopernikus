extern crate serde;
extern crate serde_derive;
extern crate yew;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

use serde_derive::Deserialize;
use yew_styles::styles;

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
        let mat1 = Material {
            id: 1,
            description: "Lorem ipsum dolor sit amet, cdiam vulputate ut. Malesuada fames ac turpis egestas. Montes nascetur ridiculus mus mauris vitae ultricies leo. Tempus egestas sed sed risus pretium quam vulputate dignissim suspendisse. Ac orci phasellus egestas tellus rutrum tellus pellentesque eu. In dictum non consectetur a erat. Quis imperdiet massa tincidunt nunc pulvinar sapien et ligula ullamcorper. Nunc vel risus commodo viverra maecenas accumsan lacus vel. Fermentum leo vel orci porta non.".to_string(),
            name: "mat1".to_string(),
            num_available: 1,
            pic: Some("http://www.jurtenland.de/images/stories/material/kohte%20grundmuster.jpg".to_string())
        };
        let mat2 = Material {
            id: 2,
            description: "".to_string(),
            name: "mat2".to_string(),
            num_available: 0,
            pic: None
        };

        let mats = vec![mat1, mat2];

        Self {
            link,
            materialien: Some(mats),
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
                        <div id="ham">
                            <lable>{"≡"}</lable>
                        </div>
                    </div>
                    <div id="util-bar"></div>
                </div>
                <div id="greeter"></div>
                {self.get_articles()}
                <div class=classes!("footer")></div>
            </div>
        }
    }
}

impl Materialverlei {

    fn get_articles(&self) -> Html {
        if let Some(mats) = &self.materialien {
            html!{
                <div id="content">
                    {
                        for mats.iter().map(|mat: &Material| {
                            let pic = mat.pic.clone().unwrap_or("https://static.thenounproject.com/png/15022-200.png".to_string());
                            
                            let available = match mat.num_available {
                                0 => {
                                    html!{
                                        <lable style="color: rgb(255, 3, 3)">{"Currently unavailable"}</lable>
                                    }
                                }
                                n => {
                                    html!{
                                        <lable style="color: rgb(0, 165, 0)">{format!("{n} available")}</lable>
                                    }
                                }
                            };
                            
                            html!{
                                <div class=classes!("article")>
                                    <div class=classes!("pic") style=format!("background-image: url({pic});")/>
                                    <div class=classes!("col2")>
                                        <div class=classes!("name")><lable>{mat.name.clone()}</lable></div>
                                        <div class=classes!("desc")><p>{mat.description.clone()}</p></div>
                                    </div>
                                    <div class=classes!("availability")>{available}</div>
                                </div>
                            }
                        })      
                    }
                </div>
            }
        } else {
            html! {
                <div id="err-no-articles">
                    <lable>{"There appear to be no articles ..."}</lable>
                </div>
            }
        }
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Material {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub pic: Option<String>,
    pub num_available: u32,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Materialverlei>::new().mount_to_body();
}

fn main(){}