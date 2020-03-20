#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate diesel;
// #[macro_use]
extern crate mongodb;
// #[macro_use]
extern crate bson;
extern crate csv;
extern crate dotenv;
extern crate uuid;

// use rocket_contrib::json::Json;
// use rocket_contrib::serve::StaticFiles;

// use rocket::http::Status;
// use rocket::response::status::Custom;
// use rocket::Data;

// use csv::Reader;

// use uuid::Uuid;

mod helpers;
mod controllers;
mod template;
mod database;

use controllers::controllers::controller::engine;

fn main() {
    engine();
}
