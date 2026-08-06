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

        for col in &self.columns {
            builder.push(format!("{} = ", col.name));
            match &col.value {
                Some(val) => {
                    if let serde_json::Value::Number(v) = val {
                        if let Some(i) = v.as_i64() {
                            builder.push_bind(i);
                        } else if let Some(f) = v.as_f64() {
                            builder.push_bind(f);
                        }
                        //builder.push_bind(v.as_i64().unwrap());
                    } else if let serde_json::Value::String(v) = val {
                        builder.push_bind(v.to_string());
                    }
                }
                None => {}
            }
        }
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
        builder.push(" RETURNING *");
        builder
    }
}
