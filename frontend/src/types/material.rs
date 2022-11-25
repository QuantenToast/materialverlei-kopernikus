use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use yew::Properties;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pic: Option<String>,
    pub num_available: u32,
    pub borrower: Option<String>,
    pub borrow_time: Option<(NaiveDate, NaiveDate)>,
    pub damage: Option<String>,
}

#[derive(PartialEq, Properties)]
pub struct MaterialComponentProps {
    pub material: Material,
}

#[derive(Serialize, Deserialize, PartialEq, Properties, Clone)]
pub struct MaterialListComponentProps {
    pub materialien: Vec<Material>,
}
