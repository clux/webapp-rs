#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!("./migrations");

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
    let p = pool::init(&database_url);
    embedded_migrations::run(&*p.clone().get().expect("connection instance"))
      .expect("Could run migrations");
    rocket::ignite()
        .manage(p)
        .mount("/", routes![health, get_posts, get_post, create_post, delete_post, update_post])
        .launch();
}
