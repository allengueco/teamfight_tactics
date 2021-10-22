use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Set5Unit {
    pub name: String,
    pub champion_id: String,
    pub cost: u8,
    pub traits: Vec<String>,
}
