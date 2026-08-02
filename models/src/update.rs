use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::{Column, Criteria};

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Update {
    pub table: String,
    pub columns: Vec<Column>,
    pub criteria: Option<Vec<Criteria>>,
}

impl Update {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE ");
        builder.push(self.table.to_string());
        builder.push(" SET ");
        builder
    }
}
