#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;
// extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
// extern crate mongodb;
// #[macro_use]
// extern crate bson;
// extern crate csv;
// extern crate dotenv;
// extern crate uuid;
// extern crate crypto;
extern crate lettre;
extern crate lettre_email;
extern crate native_tls;
// extern crate email;

// mod helpers;
// mod controllers;
// mod template;
// mod database;
// mod authorization;
// mod middleware;
// mod users;
mod mail;

// use controllers::controllers::controller::engine;

use crate::mail::models::mod_sender::{
    Sender,
    EMailService,
    ReceiveItem,
    EMailTemplate,
};
use crate::mail::sender::sender_mail::send_mail;

fn main() {

    let mut sender = Sender {
        from: EMailService {
            host    : String::from("p3plcpnl0551.prod.phx3.secureserver.net"),
            name    : String::from("Rafael"),
            email   : String::from("rafael@imsheldon.com"),
            password: String::from("sheldon123"),
        },
        to: vec![
            ReceiveItem {
                name  : String::from("Rbm91"),
                email : String::from("test_rbm@yopmail.com"),
                fails : Default::default(),
                sended: false,
            }
        ],
        mail: EMailTemplate {
            subject : String::from("Test Rust Sending mail.!"),
            template: String::from("<h1>Hi, Rust (Y)</h1>"),
        }
    };

    let receives = sender.clone().to;
    let mut i: usize = 0;

    for receive in receives {
        send_mail(&mut sender, receive, i);
        i += 1;
    }

    println!("{:?}", sender);

    // engine();
}
