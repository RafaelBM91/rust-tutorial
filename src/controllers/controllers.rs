pub mod controller {
    // --------------- //
    use rocket_contrib::serve::StaticFiles;
    use mongodb::{ Client, Database, Collection };
    use crate::database::connection::get_connection::connect;
    use crate::users::routes::routes::routes_fn::user_routes_fn;
    use crate::template::routes::routes::routes_fn::template_routes_fn;
    // --------------- //

    pub fn engine () {
        let client  : Client     = connect();
        let database: Database   = client.database("test");
        let template: Collection = database.collection("templates");
        let user    : Collection = database.collection("users");
        let collections: Vec<Collection> = vec![template, user];

        rocket::ignite()
            .manage(collections)
            .mount("/", StaticFiles::from("view"))
            .mount("/api", template_routes_fn())
            .mount("/user", user_routes_fn())
            .launch();
    }
}
