use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

use crate::enums;

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct Join {
    pub from: ReferenceFrom,
    pub to: ReferenceTo,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceFrom {
    pub table: String,
    pub column: String,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct ReferenceTo {
    pub join_type: enums::JoinType,
    pub table: String,
    pub column: String,
}
