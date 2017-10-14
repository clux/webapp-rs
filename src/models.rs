use schema::posts;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// apparently this can be done without heap storage, but lifetimes spread far..
#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
