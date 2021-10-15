use lib_tft_parse::{TraitStyle, TraitType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Set5Trait {
    key: String,
    name: String,
    description: String,
    #[serde(rename = "type")]
    type_: TraitType,
    sets: Vec<TraitStyle>,
}
