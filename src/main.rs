#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;
#[macro_use]
extern crate bson;
extern crate csv;
extern crate dotenv;
extern crate uuid;

mod helpers;
mod controllers;
mod template;
mod database;

use controllers::controllers::controller::engine;

fn main() {
    engine();
}
