use dotenv::dotenv;
use std::env;
use std::vec::Vec;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use rocket_contrib::json::Json;

use super::model::{CreatePost, NewPost, Post};

use super::schema::posts;

pub fn connection_db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(_create_post: &Json<CreatePost>) -> Post {
    let new_post = NewPost {
        title: &_create_post.title,
        body: &_create_post.body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection_db())
        .expect("Error saving new post")
}

pub fn find_post() -> Vec<Post> {
    use super::schema::posts::dsl::posts;
    use super::schema::posts::dsl::published;

    posts
        .filter(published.eq(false))
        .limit(5)
        .load::<Post>(&connection_db())
        .expect("Error loading posts")
}

pub fn update_post(id: i32) -> Post {
    use super::schema::posts::dsl::posts;
    use super::schema::posts::dsl::published;

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection_db())
        .expect(&format!("Unable to find post {}", id))
}

pub fn delete_post(id: i32) -> usize {
    use super::schema::posts::dsl::posts;

    diesel::delete(posts.find(id))
        .execute(&connection_db())
        .expect("Error deleting")
}
