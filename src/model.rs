// #[derive(Serialize, Deserialize, Debug, Queryable)]
// pub struct Model {
//     pub id: Option<String>,
//     pub name: String,
//     pub age: i32
// }

// impl Model {
//     pub fn update(&mut self, new_id: String) {
//         self.id = Some(new_id);
//     }
// }

// use super::schema::models;

// #[derive(Insertable)]
// #[table_name="models"]
// pub struct NewModel<'a> {
//     pub id: &'a str,
//     pub name: &'a str,
//     pub age: &'a i32
// }

// #[derive(Queryable)]
// pub struct ModelQuery {
//     pub id: String,
//     pub name: String,
//     pub age: i32
// }

//-------------------------------------------------------//

use super::schema::posts;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost <'a> {
    pub title: &'a str,
    pub body: &'a str
}

#[derive(Deserialize)]
pub struct ReqIDPost {
    pub id: i32
}

#[derive(Serialize)]
pub struct RespDelete {
    pub deleted: String
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorLoadDetail {
    pub line   : u64,
    pub cell   : u64,
    pub message: String
}

#[derive(Serialize)]
pub struct CustomResponse {
    pub message: String,
    pub inserted: u64,
    pub fail    : Vec<Option<ErrorLoadDetail>>
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    pub rank: usize,
}
