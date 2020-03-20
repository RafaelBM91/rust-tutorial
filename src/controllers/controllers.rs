pub mod controller {
    
    // --------------- //
    use rocket::{
        http::Status,
        State,
        Data,
        response::status::Custom
    };
    use rocket_contrib::{
        serve::StaticFiles,
        json::Json
    };
    use mongodb::{ Client, Database, Collection };
    use crate::template::app::app::fn_app_template;
    use crate::database::connection::get_connection::connect;
    use crate::helpers::models::csv::csv::mods_csv::CustomResponse;
    use crate::template::data::model::dt_template::DTemplate;
    // --------------- //

    #[post("/upload", format = "application/octet-stream", data = "<data>")]
    fn upload (data: Data, state: State<Collection>) -> Custom<Json<CustomResponse>> {
        let response = fn_app_template::upload(data, state.inner());
        Custom(
            Status::Accepted,
            Json(CustomResponse {
                message: format!("elements inserts {}", response.0.len()),
                inserted: response.0.len() as u64,
                fail: response.1,
            }),
        )
    }

    #[post("/find", format = "application/json")]
    fn find (state: State<Collection>) -> Json<Vec<DTemplate>> {
        Json(fn_app_template::find(state.inner()))
    }

    pub fn engine () {
        let client    : Client     = connect();
        let database  : Database   = client.database("test");
        let collection: Collection = database.collection("template");

        rocket::ignite()
            .manage(collection)
            .mount("/", StaticFiles::from("view"))
            .mount("/api", routes![upload,find])
            .launch();
    }

}
