use serde::{Deserialize, Serialize};
use serde_json::*;
use async_graphql::InputObject;
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct Column {
    pub name: String,
    pub alias: Option<String>,
    pub function: Option<String>,
    pub value: Option<Value>,
}

impl Column {
    pub fn build(&self, build_type: String) -> String {
        println!("build type: {}", build_type);
        return self.name.to_string();
    }
}