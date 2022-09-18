use serde_derive::{Deserialize, Serialize};
use yew::Properties;

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
