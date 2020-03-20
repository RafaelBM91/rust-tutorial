pub mod controller {
    
    // --------------- //
    use rocket::{ http::Status, State, Data };
    use rocket_contrib::serve::StaticFiles;
    use mongodb::{ Client, Database, Collection };
    use crate::template::app::app::fn_app_template;
    use crate::database::connection::get_connection::connect;
    // --------------- //

    #[post("/upload", format = "application/octet-stream", data = "<data>")]
    fn upload (data: Data, state: State<Collection>) -> Status {
        fn_app_template::upload(data, state.inner());
        Status::Accepted
    }

    pub fn engine () {
        let client    : Client     = connect();
        let database  : Database   = client.database("test");
        let collection: Collection = database.collection("template");

        rocket::ignite()
            .manage(collection)
            .mount("/", StaticFiles::from("view"))
            .mount("/api", routes![upload])
            .launch();
    }

}
