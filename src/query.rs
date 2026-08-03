use std::collections::HashMap;

use async_graphql::*;
use axum::http::HeaderMap;
use models::{Count, Delete, Insert, Select, Update};
use serde_json::Value;

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
    async fn select(
        &self,
        _ctx: &Context<'_>,
        args: Select,
    ) -> async_graphql::Result<Vec<HashMap<String, Value>>> {
        let sql = args.build().sql();
        println!("SQL: {}", sql.as_str());
        let helper = Helper::new();
        let data: Vec<HashMap<String, Value>> = helper
            .read(args.build())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect::<HashMap<String, Value>>()
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