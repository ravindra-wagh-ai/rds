use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::{Criteria, Join};
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Count {
    pub table: Option<String>,
    pub criteria: Option<Vec<Criteria>>,
    pub joins: Option<Vec<Join>>,
}

impl Count {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut builder = sqlx::QueryBuilder::new("");

        builder.push(String::from("SELECT "));
        builder.push(String::from("COUNT(1)"));
        builder.push(String::from(" FROM "));

        match &self.joins {
            Some(v) => {
                let mut join: Vec<String> = Vec::new();
                for item in v {
                    join.push(item.from.table.to_string());
                    join.push(item.to.join_type.as_str().to_string());
                    join.push(item.to.table.to_string());
                    join.push(String::from("ON"));
                    join.push(format!("{}.{}", item.to.table, item.to.column));
                    join.push(String::from("="));
                    join.push(format!("{}.{}", item.from.table, item.from.column));
                }
                //println!("{}", join.join(" "));
                builder.push(join.join(" "));
            }
            None => {
                builder.push(self.table.as_ref().unwrap_or(&String::from("")).to_string());
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

        return builder;
    }
}
