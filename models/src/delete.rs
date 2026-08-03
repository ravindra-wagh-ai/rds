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
        match &self.criteria {
            Some(v) => {
                if v.iter().len() > 0 {
                    builder.push(String::from(" WHERE "));
                    for item in v {
                        if let Some(v) = &item.lop {
                            builder.push(format!(" {} ", v.as_str().to_string()));
                        }
                        builder.push(format!("{}.{}", item.table, item.column));
                        builder.push(format!(" {} ", item.cop.as_str().to_string()));

                        if let serde_json::Value::Number(v) = &item.value {
                            builder.push_bind(v.as_i64().unwrap());
                        } else if let serde_json::Value::String(v) = &item.value {
                            builder.push_bind(v.to_string());
                        }
                        /*
                           builder.push(" AND bio LIKE CONCAT('%', ");
                           builder.push_bind(user_input);
                           builder.push(", '%')");
                        */
                    }
                }
            }
            None => {}
        }
        builder
    }
}
