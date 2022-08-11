extern crate serde;
extern crate serde_derive;
extern crate yew;

use yew::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub pic: Option<String>,
    pub num_available: u32,
}

#[derive(PartialEq, Properties)]
struct MaterialComponentProps {
    pub material: Material,
}

#[derive(PartialEq, Properties, Clone)]
struct MaterialListComponentProps {
    pub materialien: Vec<Material>,
}

#[function_component(MaterialComponent)]
fn material_component(props: &MaterialComponentProps) -> Html {
    let mat = props.material.clone();
    let pic = mat
        .pic
        .clone()
        .unwrap_or("https://static.thenounproject.com/png/15022-200.png".to_string());
    let available = match mat.num_available {
        0 => {
            html! {
                <lable style="color: rgb(255, 3, 3)">{"Currently unavailable"}</lable>
            }
        }
        n => {
            html! {
                <lable style="color: rgb(0, 165, 0)">{format!("{n} available")}</lable>
            }
        }
    };

    html! {
        <div class="article">
            <div class="pic" style={format!("background-image: url({pic});")}></div>
            <div class="col2">
                <div class="name"><lable>{mat.name.clone()}</lable></div>
                <div class="desc"><p>{mat.description.clone()}</p></div>
            </div>
            <div class="availability">{available}</div>
        </div>
    }
}

#[function_component(MaterialListComponent)]
fn material_list_component(props: &MaterialListComponentProps) -> Html {
    let mats = props.materialien.clone();
    html! {
        <div id="content">
            {
                for mats.iter().map(|mat: &Material| {
                    html!{
                        <MaterialComponent material={mat.clone()} />
                    }
                })
            }
        </div>

    }
}

#[function_component(App)]
fn app_component() -> Html {
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
        pic: None,
    };

    let mats = vec![mat1, mat2];

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
            <MaterialListComponent materialien={mats.clone()} />
            <div class="footer"/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
