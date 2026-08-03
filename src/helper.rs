use serde_json::{Value, json};
use sqlx::{Column, PgPool, Postgres, QueryBuilder, Row, TypeInfo, ValueRef, postgres::PgRow};
use std::{collections::HashMap, env};

pub struct Helper;

impl Helper {
    pub fn new() -> Self {
        Self
    }

    async fn initialize(&self) -> Result<PgPool, sqlx::Error> {
        // 1. --- BEST PRACTICE: Read connection string from environment variables ---
        let database_url = env::var("DATABASE_URL")
            .map_err(|error| sqlx::Error::Configuration(Box::new(error)))?;
        let pool = PgPool::connect(&database_url).await?;

        Ok(pool)
    }

    fn row_to_map(&self, row: &PgRow) -> HashMap<String, Value> {
        let mut map = HashMap::new();

        for column in row.columns() {
            let name = column.name();

            // 1. Get raw, type-safe value reference
            let raw_val = row.try_get_raw(name).unwrap();

            if raw_val.is_null() {
                map.insert(name.to_string(), Value::Null);
                continue;
            }

            // 2. Safely match based on database type naming
            let type_name = column.type_info().name();
            /* let json_val = match type_name {
                "INT2" | "SMALLINT" | "INT4" | "INT" | "INTEGER" => {
                    let v: i32 = row.get(name);
                    Value::Number(v.into())
                }
                "INT8" | "BIGINT" => {
                    let v: i64 = row.get(name);
                    Value::Number(v.into())
                }
                "FLOAT4" | "REAL" => {
                    let v: f32 = row.get(name);
                    Value::Number(v.into())
                }
                "FLOAT8" | "DOUBLE PRECISION" => {
                    let v: f64 = row.get(name);
                    Value::Number(v.into())
                }
                "BOOL" | "BOOLEAN" => {
                    let v: bool = row.get(name);
                    Value::Bool(v)
                }
                "VARCHAR" | "CHAR" | "TEXT" | "NAME" => {
                    let v: String = row.get(name);
                    Value::String(v)
                }
                "JSON" | "JSONB" => {
                    let v: Value = row.get(name);
                    v
                }
                _ => {
                    // Fallback to text representation for unmapped types (UUIDs, Dates, Timestamps)
                    let v: Result<String, _> = row.try_get(name);
                    Value::String(v.unwrap_or_else(|_| "Unsupported type mapping".to_string()))
                }
            }; */
            let json_val = match type_name {
                 "INT2" | "SMALLINT" => {
                    let v: i16 = row.get(name);
                    json!(v)
                }
                "INT4" | "INT" | "INTEGER" => {
                    let v: i32 = row.get(name);
                    json!(v)
                }                
                "INT8" | "BIGINT" => {
                    let v: i64 = row.get(name);
                    json!(v)
                }
                "FLOAT4" | "REAL" => {
                    let v: f32 = row.get(name);
                    json!(v)
                }
                "FLOAT8" | "DOUBLE PRECISION" => {
                    let v: f64 = row.get(name);
                    json!(v)
                }
                "BOOL" | "BOOLEAN" => {
                    let v: bool = row.get(name);
                    json!(v)
                }
                "VARCHAR" | "CHAR" | "TEXT" | "NAME" => {
                    let v: String = row.get(name);
                    json!(v)
                }
                "JSON" | "JSONB" => {
                    let v: Value = row.get(name);
                    v
                }
                _ => {
                    // Fallback to text representation for unmapped types (UUIDs, Dates, Timestamps)
                    let v: Result<String, _> = row.try_get(name);
                    json!(v.unwrap_or_else(|_| "Unsupported type mapping".to_string()))
                }
            };
            map.insert(name.to_string(), json_val);
        }

        map
    }

    pub async fn read(&self, mut builder: QueryBuilder<Postgres>) -> Option<Vec<HashMap<String, Value>>> {

        let mut list: Vec<HashMap<String, Value>> = Vec::new();

        let pool = self.initialize().await.unwrap();
        //let mut sql = args.build();
        let query = builder.build();
        let data = query.fetch_all(&pool).await.ok();
        for row in data.as_ref().unwrap_or(&Vec::new()) {
            let map = self.row_to_map(row);
            list.push(map);
        }
        Some(list)
    }

    pub async fn _write(&self, mut builder: QueryBuilder<Postgres>) -> Option<Vec<HashMap<String, Value>>> {
        let pool = self.initialize().await.unwrap();
        // For write operations we execute the query. execute() returns a PgQueryResult
        // which does not contain rows to map. Return an empty list on success.
        let query = builder.build();
        let _res = query.execute(&pool).await.ok()?;
        Some(Vec::new())
    }
    
    /* pub async fn convert(&self, data: Option<Vec<PgRow>>) -> Option<Vec<HashMap<String,Value>>> {
        let list: Vec<HashMap<String,Value>> = Vec::new();
        Some(list);
    } */
}
