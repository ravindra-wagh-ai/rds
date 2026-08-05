use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::Criteria;
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Max {
    pub table: String,
    pub column: String,
    pub criteria: Option<Vec<Criteria>>,
}

impl Max {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut builder = sqlx::QueryBuilder::new("SELECT ");

        builder.push(format!("MAX({}.{}) as max", self.table, self.column));
        builder.push(format!(" FROM {} ", self.table));

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
                            if let Some(i) = v.as_i64() {
                                builder.push_bind(i);
                            } else if let Some(f) = v.as_f64() {
                                builder.push_bind(f);
                            }

                            //builder.push_bind(v.as_i64().unwrap());
                        } else if let serde_json::Value::String(v) = &item.value {
                            builder.push_bind(v.to_string());
                        }
                    }
                }
            }
            None => {}
        }

        //println!("{}", builder.sql().as_str());
        return builder;
    }
}
