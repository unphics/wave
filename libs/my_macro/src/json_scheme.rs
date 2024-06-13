use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Default, Serialize, Deserialize)]
struct scheme {
    id: Option<i32>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, scheme>>
}