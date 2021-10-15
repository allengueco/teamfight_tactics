use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set5Unit {
    name: String,
    champion_id: String,
    cost: u8,
    traits: Vec<String>,
}
