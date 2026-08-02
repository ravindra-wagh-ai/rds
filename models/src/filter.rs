use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::Criteria;

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct FilterData {
    pub criteria: Option<Vec<Criteria>>,
    pub offset: Option<i64>,
    pub limit: Option<i32>,
}