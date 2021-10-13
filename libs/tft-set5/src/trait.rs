use serde::{Deserialize, Serialize};
use lib_tft_parse::{TraitType, TraitStyle};


#[derive(Deserialize, Serialize)]
pub struct Set5Trait {
    key: String,
    name: String,
    description: String,
    #[serde(rename = "type")]
    type_: TraitType,
    sets: Vec<TraitStyle>,
}