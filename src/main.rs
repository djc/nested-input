use async_graphql::http::graphiql_source;
use async_graphql::{EmptySubscription, InputObject, Object, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response::{self, IntoResponse};
use axum::routing::get;
use axum::{Router, Server};

async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(graphiql_source("/", None))
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn foo(&self) -> &'static str {
        "foo"
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn edit_foo(&self, _input: EditFooInput) -> Result<Option<Foo>, axum::Error> {
        todo!()
    }
}

#[derive(InputObject)]
struct EditFooInput {
    info: Option<Info>,
}

#[derive(InputObject)]
pub struct Info {
    email: Option<String>,
}

#[Object]
impl Info {
    async fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

pub struct Foo {
    info: Option<Info>,
}

#[Object]
impl Foo {
    async fn info(&self) -> Option<&Info> {
        self.info.as_ref()
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
