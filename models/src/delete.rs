use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::Criteria;

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Delete {
    pub table: String,
    pub criteria: Option<Vec<Criteria>>,
}

impl Delete {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("DELETE FROM ");
        builder.push(self.table.to_string());
        builder
    }
}
