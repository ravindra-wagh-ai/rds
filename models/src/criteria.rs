use serde::{Deserialize, Serialize};
use serde_json::*;
use async_graphql::InputObject;

use crate::enums;
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Criteria {
    pub table: String,
    pub column: String,
    pub cop: enums::Cop,
    pub lop: Option<enums::Lop>,
    pub value: Value,
}
