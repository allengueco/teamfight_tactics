use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Set5Item {
    id: usize,
    name: String,
    description: String,
    is_unique: bool,
    is_elusive: bool,
    is_radiant: bool,
}
