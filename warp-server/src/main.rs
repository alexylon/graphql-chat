use std::convert::Infallible;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_warp::{graphql_subscription, GraphQLResponse, GraphQLWebSocket};
use chats::{Mutation, Query, Storage, Subscription};
use warp::{http::Response as HttpResponse, Filter};

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(vec!["GET", "POST", "DELETE"]);

    let schema = Schema::build(Query, Mutation, Subscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema.clone())
        .and_then(
        |(schema, request): (
            Schema<Query, Mutation, Subscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    ).with(&cors);

    let graphql_playground = warp::path::end()
        .and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
            ))

    }).with(&cors);

    let routes = graphql_subscription(schema)
        .or(graphql_playground)
        .or(graphql_post);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
