use diesel;
use rocket_contrib::Json;
use diesel::prelude::*;

use super::pool::DbConn;
use super::models::*;
use super::schema::posts::dsl::*;
use super::schema::posts;

// ----------------------------------------------------------------------------
// crud routes
// ----------------------------------------------------------------------------

#[get("/posts")]
pub fn get_posts(conn: DbConn) -> QueryResult<Json<Vec<Post>>> {
    posts.filter(published.eq(true))
        .order(id.desc())
        .limit(5)
        .load::<Post>(&*conn)
        .map(|xs| Json(xs))
}

#[get("/posts/<postid>")]
pub fn get_post(postid: i32, conn: DbConn) -> QueryResult<Json<Post>> {
    posts.find(postid)
        .get_result::<Post>(&*conn)
        .map(|x| Json(x))
}

#[put("/posts/<postid>")]
pub fn update_post(postid: i32, conn: DbConn) -> QueryResult<Json<Post>> {
    diesel::update(posts.find(postid))
        .set(published.eq(true))
        .get_result::<Post>(&*conn)
        .map(|x| Json(x))
}

#[post("/posts", data="<newpost>")]
pub fn create_post(newpost: Json<NewPost>, conn: DbConn) -> QueryResult<Json<Post>> {
    diesel::insert(&newpost.0)
        .into(posts::table)
        .get_result(&*conn)
        .map(|x| Json(x))
}

#[delete("/posts/<postid>")]
pub fn delete_post(postid: i32, conn: DbConn) -> QueryResult<Json<usize>> {
    diesel::delete(posts.filter(id.eq(postid)))
        .execute(&*conn)
        .map(|x| Json(x))
}
