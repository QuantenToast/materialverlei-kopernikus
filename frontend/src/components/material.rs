extern crate serde;
extern crate serde_derive;
extern crate yew;

use yew::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pic: Option<String>,
    pub num_available: u32,
}

#[derive(PartialEq, Properties)]
pub struct MaterialComponentProps {
    pub material: Material,
}

#[derive(Serialize, Deserialize, PartialEq, Properties, Clone)]
pub struct MaterialListComponentProps {
    pub materialien: Vec<Material>,
}

#[function_component(MaterialComponent)]
pub fn material_component(props: &MaterialComponentProps) -> Html {
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
pub fn material_list_component(props: &MaterialListComponentProps) -> Html {
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
