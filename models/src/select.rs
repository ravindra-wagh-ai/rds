use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder};

use crate::{Criteria, Join, Table};
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Select {
    pub tables: Vec<Table>,
    pub criteria: Option<Vec<Criteria>>,
    pub joins: Option<Vec<Join>>,
    pub offset: Option<i64>,
    pub limit: Option<i32>,
}

impl Select {
    pub fn build(&self) -> QueryBuilder<Postgres> {
        let mut tables: Vec<String> = Vec::new();
        let mut column: Vec<String> = Vec::new();
        let mut builder = sqlx::QueryBuilder::new("");

        for table in &self.tables {
            tables.push(table.name.to_string());
            for col in &table.columns {
                match &col.function {
                    Some(v) => match &col.alias {
                        Some(a) => {
                            column.push(format!(
                                "{}({}.{}) AS {}",
                                v.clone(),
                                table.name,
                                col.name,
                                a.clone()
                            ));
                        }
                        None => {
                            column.push(format!("{}({}.{})", v.clone(), table.name, col.name));
                        }
                    },
                    None => match &col.alias {
                        Some(v) => {
                            column.push(format!("{}.{} AS {}", table.name, col.name, v.clone()));
                        }
                        None => {
                            column.push(format!("{}.{}", table.name, col.name));
                        }
                    },
                }
            }
        }

        builder.push(String::from("SELECT "));
        builder.push(column.join(", "));
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
                builder.push(tables.join(", "));
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

        match &self.offset {
            Some(v) => {
                builder.push(String::from(" OFFSET "));
                builder.push_bind(v);
            }
            None => {}
        }

        match &self.limit {
            Some(v) => {
                builder.push(String::from(" LIMIT "));
                builder.push_bind(v);
            }
            None => {}
        }
        //println!("{}", builder.sql().as_str());
        return builder;
    }
}
