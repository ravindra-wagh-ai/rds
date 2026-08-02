use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::Column;

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}
