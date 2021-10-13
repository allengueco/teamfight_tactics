use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Set5Items {
    id: u8,
    name: String,
    description: String,
    is_unique: bool,
    is_elusive: bool,
    is_radiant: bool,
}

