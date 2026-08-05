use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::Criteria;
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Avg {
    pub table: String,
    pub column: String,
    pub criteria: Option<Vec<Criteria>>,
}

impl Avg {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut builder = sqlx::QueryBuilder::new("SELECT ");

        builder.push(format!("AVG({}.{})::FLOAT8 as avg", self.table, self.column));
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
                        println!("Value: {:?}", &item.value);
                        builder.push_bind(&item.value.clone());

                        /* if let serde_json::Value::Number(v) = &item.value {
                            if let Some(i) = v.as_i64() {
                                builder.push_bind(&item.value.clone());
                            } else if let Some(f) = v.as_f64() {
                                builder.push_bind(&item.value.clone());
                            }
                            //builder.push_bind(v.as_i64().unwrap());
                        } else if let serde_json::Value::String(v) = &item.value {
                            builder.push_bind(v.to_string());
                        } */
                    }
                }
            }
            None => {}
        }

        //println!("{}", builder.sql().as_str());
        return builder;
    }
}
