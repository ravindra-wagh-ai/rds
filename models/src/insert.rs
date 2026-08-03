use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::*;
use sqlx::{Postgres, QueryBuilder};

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Insert {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<Value>,
}

impl Insert {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO ");
        builder.push(self.table.to_string());
        builder.push(" (");
        //builder.push_comma_separated(&self.columns);
        builder.push(") VALUES (");
        builder.push(&self.columns.join(", "));
        builder.push(")");
        for value in &self.values {
            builder.push_bind(value);
        }
        builder
    }
}
