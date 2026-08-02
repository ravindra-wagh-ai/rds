use std::env;
use dotenvy::dotenv;
use async_graphql::{http::GraphiQLSource, *};
use async_graphql_axum::*;
use axum::{
    Router, extract::State, http::HeaderMap, response::{Html, IntoResponse},
};
mod helper;


mod query;
pub type AppSchema = Schema<query::QueryRoot, EmptyMutation, EmptySubscription>;

async fn handler(
    State(schema): State<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    //println!("Headers: {:?}", headers);
    let mut req = req.into_inner();
    req = req.data(headers);
    schema.execute(req).await.into()
}

async fn playground() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").expect("3000");
    println!("Starting server on port: {}", port);
    let schema = Schema::build(query::QueryRoot, EmptyMutation, EmptySubscription).finish();
    let app = Router::new()
        .route(
            "/",
            axum::routing::get(playground).post(handler),
        )
        .with_state(schema);

    println!("GraphQL server running on http://0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
