use std::fmt::Display;

use chrono::{DateTime, Utc};
use juniper::{EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject, RootNode, ScalarValue, graphql_object};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use serde::{Deserialize, Serialize};

//use super::schema::posts::dsl::*;
use super::schema::users::dsl::*;
//use super::schema::posts::*;
use super::schema::users;

// #[derive(GraphQLObject)]
// #[derive(Queryable)]
// struct Post {
//     id: i32,
//     title: String,
//     post: String,
//     score: i32,
//     author: User,
//     create_date: DateTime<Utc>,
//     update_date: DateTime<Utc>
// }

// #[derive(GraphQLInputObject)]
// //#[derive(Insertable)]
// //#[table_name="posts"]
// struct NewPost {
//     title: String,
//     post: String,
//     author: i32
// }

#[derive(GraphQLObject, Deserialize, Serialize, Queryable, Debug, Clone)]
//#[table_name = "users"]
struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub pass: String,
    pub create_date: DateTime<Utc>
}
#[derive(GraphQLInputObject, Deserialize, Serialize, Insertable, Debug, Clone)]
#[table_name="users"]
struct NewUser {
    pub username: String,
    pub email: String,
    pub pass: String
}


pub struct Context {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}


impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(
    context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn findUser(context: &Context, user_id: i32) -> FieldResult<User> {

        let connection = context.pool.get()?;
        //let result = users.filter(id.eq(userId)).load::<User>(&connection).expect("No user found");
        //let results = users::table.find(user_id).get_result::<User>(&connection);
        let result = users.find(user_id).load::<User>(&*connection).expect("Whoops");
        //let result2 = result.into()?;
        //if result.len() == 1 {
        Ok(result.get(0).unwrap().clone())
        //} else {
         //   Err("No user found")
        //}
        
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
    fn newUser(context: &Context, new_user: NewUser) -> FieldResult<User, S> {

        let connection = context.pool.get()?;

        //users.insert_into(users::table).values(&new_user).get_result(&*connection).expect("Whoops");

        let result = new_user.insert_into(users::table).load::<User>(&*connection).expect("Whoops");

        Ok(result.get(0).unwrap().clone())
    }
}


pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}

pub fn create_context() -> Context {
    dotenv().ok();
    let database_url =  env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Context {
        pool: r2d2::Pool::builder().build(ConnectionManager::<PgConnection>::new(&database_url)).expect(&format!("Error connecting to {}", database_url))
    }
}