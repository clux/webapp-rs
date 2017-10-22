use diesel;
use r2d2;

// for request guard
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

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
pub fn init(database_url: &str) -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<DbType>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}
