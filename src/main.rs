#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate r2d2_diesel;
extern crate r2d2;

mod schema;
mod models;
mod pool;
mod routes;

use self::routes::*;

// ----------------------------------------------------------------------------

fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(pool::init(&database_url))
        .mount("/", routes![get_posts, get_post, create_post, delete_post, update_post])
        .launch();
}
