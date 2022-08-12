extern crate serde;
extern crate serde_derive;
extern crate yew;

use yew::prelude::*;

mod components;
use components::material::*;

#[function_component(App)]
fn app_component() -> Html {
    let mat1 = Material {
        id: "1".to_string(),
        description: "Lorem ipsum dolor sit amet, cdiam vulputate ut. Malesuada fames ac turpis egestas. Montes nascetur ridiculus mus mauris vitae ultricies leo. Tempus egestas sed sed risus pretium quam vulputate dignissim suspendisse. Ac orci phasellus egestas tellus rutrum tellus pellentesque eu. In dictum non consectetur a erat. Quis imperdiet massa tincidunt nunc pulvinar sapien et ligula ullamcorper. Nunc vel risus commodo viverra maecenas accumsan lacus vel. Fermentum leo vel orci porta non.".to_string(),
        name: "mat1".to_string(),
        num_available: 1,
        pic: Some("http://www.jurtenland.de/images/stories/material/kohte%20grundmuster.jpg".to_string())
    };
    let mat2 = Material {
        id: "2".to_string(),
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
