#[macro_use]
extern crate diesel;
extern crate dotenv;

mod graphql_schema;
mod schema;

use graphql_schema::{create_schema,create_context, Schema, Context};
use rocket::{response::content, Rocket, State};


#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema.inner(), &*context)
}

#[rocket::main]
async fn main() {
    Rocket::build()
        .manage(create_context())
        .manage(create_schema())
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await
        .expect("server to launch");
}