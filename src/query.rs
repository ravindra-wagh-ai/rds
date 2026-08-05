//use std::collections::HashMap;

use async_graphql::*;
use axum::http::HeaderMap;
use models::{Avg, Count, Delete, Insert, Max, Min, Select, Sum, Update};
use serde_json::Value;
use sqlx::{Postgres, QueryBuilder};

use crate::helper::Helper;

pub struct _AuthGuard;

impl Guard for _AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let headers = ctx.data::<HeaderMap>()?;
        println!("{:?}", headers);
        if headers.contains_key("authorization") {
            Ok(())
        } else {
            Err("Missing authorization header".into())
        }
    }
}

pub fn _fields(ctx: &Context<'_>) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    if let Some(field) = ctx.field().selection_set().find(|&x| x.name() == "list") {
        for item in field.selection_set() {
            list.push(item.name().to_string());
        }
    }
    list
}

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    //#[graphql(guard = "AuthGuard")]
    async fn health(&self) -> &str {
        "I'm healthy"
    }
    async fn tables(
        &self,
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<Value>> {

        let query = "SELECT table_name as name FROM information_schema.tables WHERE table_schema='public'";
        let builder:QueryBuilder<Postgres> = QueryBuilder::new(query);

        let helper = Helper::new();
        let data: Vec<Value> = helper
            .read(builder)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect::<Value>()
            })
            .collect();
        Ok(data)
    }

    async fn columns(
        &self,
        _ctx: &Context<'_>,
         args: Option<Vec<String>>,
    ) -> async_graphql::Result<Vec<Value>> {


        let query = "SELECT table_name as table, column_name as name, data_type as type, character_octet_length length, is_nullable nullable FROM information_schema.columns WHERE table_schema ='public'";

        let mut builder:QueryBuilder<Postgres> = QueryBuilder::new(query);

        match args {
            Some(ref tables) if !tables.is_empty() => {
                builder.push(" AND table_name IN (");
                for (i, table) in tables.iter().enumerate() {
                    if i > 0 {
                        builder.push(", ");
                    }
                    builder.push_bind(table);
                }
                builder.push(")");
            }
            _ => {}
        }
        
        //println!("SQL: {}", builder.build().sql().as_str());
        
        let helper = Helper::new();
        let data: Vec<Value> = helper
            .read(builder)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect::<Value>()
            })
            .collect();
        Ok(data)
    }

    async fn select(
        &self,
        _ctx: &Context<'_>,
        args: Select,
    ) -> async_graphql::Result<Vec<Value>> {
        let sql = args.build().sql();
        println!("SQL: {}", sql.as_str());
        let helper = Helper::new();
        let data: Vec<Value> = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect::<Value>()
            })
            .collect();
        Ok(data)
    }
    async fn count(&self, _ctx: &Context<'_>, args: Count) -> async_graphql::Result<i64> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("count")
            .unwrap()
            .as_i64()
            .unwrap();
        Ok(data)
    }

    async fn min(&self, _ctx: &Context<'_>, args: Min) -> async_graphql::Result<Value> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("min").unwrap().clone();
        Ok(data)
    }

    async fn max(&self, _ctx: &Context<'_>, args: Max) -> async_graphql::Result<Value> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("max").unwrap().clone();
        Ok(data)
    }

    async fn sum(&self, _ctx: &Context<'_>, args: Sum) -> async_graphql::Result<Value> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("sum").unwrap().clone();
        Ok(data)
    }

     async fn avg(&self, _ctx: &Context<'_>, args: Avg) -> async_graphql::Result<Value> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("avg").unwrap().clone();
        println!("Avg: {:?}", data);
        Ok(data)
    }
    
}


pub struct Mutation;

#[Object]
impl Mutation {
   async fn insert(&self, _ctx: &Context<'_>, args: Insert) -> async_graphql::Result<u64> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("count")
            .unwrap()
            .as_i64()
            .unwrap();
        Ok(u64::try_from(data).unwrap_or_default())
    }

    async fn update(&self, _ctx: &Context<'_>, args: Update) -> async_graphql::Result<u64> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("count")
            .unwrap()
            .as_i64()
            .unwrap();
        Ok(u64::try_from(data).unwrap_or_default())
    }

    async fn delete(&self, _ctx: &Context<'_>, args: Delete) -> async_graphql::Result<u64> {
        let helper = Helper::new();
        let data = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .first()
            .unwrap()
            .get("count")
            .unwrap()
            .as_i64()
            .unwrap();
        Ok(u64::try_from(data).unwrap_or_default())
    }
}