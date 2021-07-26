use std::fmt::Display;

use chrono::{DateTime, Utc};
use juniper::{
    graphql_object, EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject, RootNode,
    ScalarValue,
};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use serde::{Deserialize, Serialize};

use super::schema::posts;
use super::schema::posts::dsl::*;
use super::schema::users;
use super::schema::users::dsl::*;

#[derive(GraphQLObject, Deserialize, Serialize, Queryable, Debug, Clone)]
struct Post {
    id: i32,
    title: String,
    post: String,
    score: i32,
    author: User,
    create_date: DateTime<Utc>,
    update_date: DateTime<Utc>,
}

#[derive(GraphQLInputObject, Deserialize, Serialize, Insertable, Debug, Clone)]
#[table_name = "posts"]
struct NewPost {
    title: String,
    post: String,
    author: i32,
}

#[derive(GraphQLObject, Deserialize, Serialize, Queryable, Debug, Clone)]
struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub pass: String,
    pub create_date: DateTime<Utc>,
}
#[derive(GraphQLInputObject, Deserialize, Serialize, Insertable, Debug, Clone)]
#[table_name = "users"]
struct NewUser {
    pub username: String,
    pub email: String,
    pub pass: String,
}

pub struct Context {
    pool: deadpool_diesel::postgres::Pool,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(
    context = Context,
)]
impl Query {
    async fn apiVersion() -> &str {
        "1.0"
    }

    async fn findUser(context: &Context, user_id: i32) -> FieldResult<User> {
        let connection = context.pool.get().await?;
        let result = users
            .find(user_id)
            .load::<User>(&*connection)
            .expect("Whoops");
        Ok(result.get(0).unwrap().clone())
    }
}

// Now, we do the same for our Mutation type.

pub struct Mutation;

#[graphql_object(
    context = Context,

    // If we need to use `ScalarValue` parametrization explicitly somewhere
    // in the object definition (like here in `FieldResult`), we should
    // declare an explicit type parameter for that, and specify it.
    scalar = S,
)]
impl<S: ScalarValue + Display> Mutation {
    async fn newUser(context: &Context, new_user: NewUser) -> FieldResult<User, S> {
        let connection = context.pool.get().await?;

        //users.insert_into(users::table).values(&new_user).get_result(&*connection).expect("Whoops");

        let result = new_user
            .insert_into(users::table)
            .load::<User>(&*connection)
            .expect("Whoops");

        Ok(result.get(0).unwrap().clone())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

pub fn create_context() -> Context {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //let manager = ConnectionManager::<PgConnection>::new(&database_url).expect(&format!("Error connecting to {}", database_url);
    let manager = deadpool_diesel::postgres::Manager::new(database_url);
    Context {
        pool: deadpool_diesel::postgres::Pool::new(manager, 5),
    }
}
