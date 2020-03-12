#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate csv;
extern crate uuid;

use uuid::Uuid;

use rocket_contrib::json::Json;

use rocket::Data;
use rocket::http::Status;
use rocket::response::status::Custom;

use csv::Reader;

mod model;
use model::{ Post, ReqIDPost, RespDelete, CustomResponse, CreatePost, ErrorLoadDetail };

mod schema;

mod functions;
use functions::{ create_post, find_post, update_post, delete_post };

#[get("/")]
fn home() -> &'static str {
    "Hello, world!"
}

#[post("/create", format = "application/json", data = "<_create_post>")]
fn create (_create_post: Json<CreatePost>) -> Json<Post> {
    let response: Post = create_post(&_create_post);
    Json(response)
}

#[post("/find", format = "application/json")]
fn find () -> Json<Vec<Post>> {
    let response: Vec<Post> = find_post();
    Json(response)
}

#[post("/update", format = "application/json", data = "<_update_post>")]
fn update (_update_post: Json<ReqIDPost>) -> Json<Post> {
    let response: Post = update_post(_update_post.id);
    Json(response)
}

#[post("/delete", format = "application/json", data = "<_delete_post>")]
fn delete (_delete_post: Json<ReqIDPost>) -> Json<RespDelete> {
    let responde: usize = delete_post(_delete_post.id);
    Json(RespDelete {
        deleted: format!("Items Deleted: {}", responde)
    })
}

fn read_csv (mut read: Reader<std::fs::File>) -> (Vec<CreatePost>, Vec<Option<ErrorLoadDetail>>) {
    let mut rows  : Vec<CreatePost>              = Vec::new();
    let mut errors: Vec<Option<ErrorLoadDetail>> = Vec::new();
    let mut header: csv::StringRecord            = csv::StringRecord::new();

    match read.headers() {
        Ok(headers) => header = headers.clone(),
        Err(_) => ()
    }

    for result in read.records() {
        match result {
            Ok(value) => {
                match value.deserialize::<CreatePost>(Some(&header)) {
                    Ok(value) => rows.push(value),
                    Err(err)  => {
                        let error: ErrorLoadDetail = catch_error(err).unwrap();
                        errors.push(
                            Some(error.clone())
                        );
                        if error.line == 0 {
                            break;
                        }
                    },
                }
            }
            Err(_) => (),
        }
    }
    (rows, errors)
}

fn catch_error (error: csv::Error) -> Option<ErrorLoadDetail> {
    match error.kind() {
        csv::ErrorKind::Deserialize { pos: _pos, err: _err } => {
            let position = _pos.as_ref().unwrap();
            let line    = if _err.field().is_some() { position.line() } else { 0 };
            let cell    = if _err.field().is_some() { _err.field().unwrap() + 1 } else { 0 };
            let message = _err.kind().to_string();
            Some(ErrorLoadDetail { line, cell, message })
        }
        _ => None
    }
}

#[post("/upload", format = "application/octet-stream", data = "<data>")]
fn upload (data: Data) -> Custom<Json<CustomResponse>> {
    let file_id = Uuid::new_v4();
    let file_id_str: &str = &format!("/tmp/{}.csv", file_id.to_string());
    let _stream = data.stream_to_file( file_id_str );
    let read = Reader::from_path( file_id_str ).unwrap();

    let (rows, errors) = read_csv(read);

    Custom (
        Status::Accepted,
        Json(CustomResponse {
            message: format!("elements inserts {}", rows.len()),
            inserted: rows.len() as u64,
            fail: errors
        })
    )
}

mod module;

fn main() {
    // rocket::ignite()
    //     .mount("/", routes![home, create, find, update, delete, upload])
    //     .launch();
    module::file::modules::connection(12);
}
