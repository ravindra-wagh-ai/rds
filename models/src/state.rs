use async_graphql::{SimpleObject};
use db::Helper;
use sqlx::{FromRow, Row, postgres::PgRow};

use crate::{Column, Count, FilterData, Select, Table};

#[derive(FromRow, SimpleObject, serde::Serialize, serde::Deserialize)]
pub struct State {
    #[sqlx(rename = "id")]
    pub id: i32,
    #[sqlx(rename = "name")]
    pub name: String,
}

#[derive(SimpleObject, serde::Serialize, serde::Deserialize)]
pub struct StateOutput {
    pub code: i32,
    pub succeed: bool,
    pub message: String,
    pub total: i64,
    pub list: Vec<State>,
}

impl State {
    pub fn parse(row: PgRow) -> Result<State, String> {
        let id: i32 = row
            .try_get("id")
            .map_err(|e| format!("Failed to get mandatory ID: {}", e))?;

        let name: String = row
            .try_get("name")
            .map_err(|e| format!("Failed to get mandatory Name: {}", e))?;

        let state = State { id, name };
        Ok(state)
    }
    pub async fn list(columns:Vec<Column>, filter: Option<FilterData>) -> StateOutput {
        let table_name="states";
        let mut args = Select {
            tables: Vec::new(),
            criteria: Vec::new(),
            joins: None,
            limit: None,
            offset: None,
        };

        let mut count = Count {
            tables: Vec::new(),
            criteria: Vec::new(),
            joins: None,
        };

        count.tables.push(table_name.to_string());

        let table: Table = Table {
            name: String::from(table_name.to_string()),
            columns: columns,
        };
        args.tables.push(table);

        count.criteria = FilterData::parse(filter.clone(), table_name);
        args.criteria = FilterData::parse(filter.clone(), table_name);

        let data = Helper::select(args.build()).await;
        let rows = Helper::count(count.build()).await;

        let output: StateOutput;
        
        match data {
            Some(list) => {                
                output = StateOutput {
                    code: 0,
                    succeed: true,
                    message: String::from(""),
                    total: rows,
                    list: list.into_iter().filter_map(|row| State::parse(row).ok()).collect(),
                }
            }
            None => {
                output = StateOutput {
                    code: 204,
                    succeed: false,
                    message: String::from("No data found"),
                    total: rows,
                    list: Vec::new(),
                }
            }
        };
        output
    }
}
