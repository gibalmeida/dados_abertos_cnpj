#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;

extern crate r2d2;
extern crate r2d2_diesel;
use r2d2_diesel::ConnectionManager;

use std::env;
use diesel::MysqlConnection;
use dotenv::dotenv;

use rocket::{response::content, Rocket, State};

use crate::graphql_schema::{Context, Schema};

mod graphql_schema;

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
    request.execute_sync(&*schema, &*context)
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();
       
    Rocket::build()
        .manage(Context { pool })
        .manage(graphql_schema::create_schema())
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await
        .expect("server to launch");
}
