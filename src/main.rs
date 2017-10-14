#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate diesel;

extern crate r2d2_diesel;
extern crate r2d2;

extern crate webapp;
use self::webapp::models::*;
use self::webapp::schema::posts::dsl::*;
use self::webapp::schema::posts;
use self::diesel::prelude::*;

// for request guard
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
// extra rocket stuff
use rocket_contrib::Json;

// ----------------------------------------------------------------------------
// database pooling and convenience types
// ----------------------------------------------------------------------------

// Master database selector (all others are features in Cargo.toml)
type DbType = diesel::pg::PgConnection;

use r2d2_diesel::ConnectionManager;
type Pool = r2d2::Pool<ConnectionManager<DbType>>;
type PoolConn = r2d2::PooledConnection<ConnectionManager<DbType>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
// NB: cannot implement FromRequest on PoolConn directly, so this is necessary
pub struct DbConn(pub PoolConn);

// For the convenience of using an &DbConn as a &DbType.
impl Deref for DbConn {
    type Target = DbType;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

/// Initializes a database pool via r2d2
fn init_pool() -> Pool {
    use std::env;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<DbType>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

// ----------------------------------------------------------------------------
// crud routes
// ----------------------------------------------------------------------------

#[get("/posts")]
fn get_posts(conn: DbConn) -> QueryResult<Json<Vec<Post>>> {
    posts.filter(published.eq(true))
        .order(id.desc())
        .limit(5)
        .load::<Post>(&*conn)
        .map(|xs| Json(xs))
}

#[get("/posts/<postid>")]
fn get_post(postid: i32, conn: DbConn) -> QueryResult<Json<Post>> {
    posts.find(postid)
        .get_result::<Post>(&*conn)
        .map(|x| Json(x))
}

#[put("/posts/<postid>")]
fn update_post(postid: i32, conn: DbConn) -> QueryResult<Json<Post>> {
    diesel::update(posts.find(postid))
        .set(published.eq(true))
        .get_result::<Post>(&*conn)
        .map(|x| Json(x))
}

#[post("/posts", data="<newpost>")]
fn create_post(newpost: Json<NewPost>, conn: DbConn) -> QueryResult<Json<Post>> {
    diesel::insert(&newpost.0)
        .into(posts::table)
        .get_result(&*conn)
        .map(|x| Json(x))
}

#[delete("/posts/<postid>")]
fn delete_post(postid: i32, conn: DbConn) -> QueryResult<Json<usize>> {
    diesel::delete(posts.filter(id.eq(postid)))
        .execute(&*conn)
        .map(|x| Json(x))
}

// ----------------------------------------------------------------------------

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![get_posts, get_post, create_post, delete_post, update_post])
        .launch();
}
