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
        builder.push(&self.columns.join(", "));
        //builder.push_comma_separated(&self.columns);
        builder.push(") VALUES (");
        
        for value in &self.values {
            if let serde_json::Value::Number(v) = &value {
                if let Some(i) = v.as_i64() {
                    builder.push_bind(i);
                } else if let Some(f) = v.as_f64() {
                    builder.push_bind(f);
                }
                //builder.push_bind(v.as_i64().unwrap());
            } else if let serde_json::Value::String(v) = &value {
                builder.push_bind(v.to_string());
            }
        }
        builder.push(") RETURNING *");
        builder
    }
}
