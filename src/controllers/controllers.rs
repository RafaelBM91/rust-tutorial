pub mod controller {
    // --------------- //
    use rocket::{
        http::Status,
        State,
        Data,
        response::status::Custom,
        fairing::AdHoc,
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
    use crate::template::data::model::dt_template::FindTemplateParams;
    use crate::middleware::caught_req::caught::caught_authorization;
    use crate::middleware::intercept::interceptor::AuthCaught;
    // --------------- //

    #[post("/upload", format="application/octet-stream", data="<data>")]
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

    #[post("/find", format="application/json", data="<params>")]
    fn find (
        params: Json<FindTemplateParams>,
        state : State<Collection>
    ) -> Json<Vec<DTemplate>> {
        Json(fn_app_template::find(params.into_inner(), state.inner()))
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Intercept {
        another: String
    }

    #[post("/intercept", format="application/json", data="<params>")]
    fn intercept (
        params: Json<Intercept>,
        id: AuthCaught,
    ) -> &'static str {
        println!("Result: {:?}", id);
        "Sensitive data."
    }

    pub fn engine () {
        let client    : Client     = connect();
        let database  : Database   = client.database("test");
        let collection: Collection = database.collection("template");

        rocket::ignite()
            .manage(collection)
            .mount("/", StaticFiles::from("view"))
            .mount("/api", routes![upload,find,intercept])
            .attach(AdHoc::on_request("Authorization", caught_authorization))
            .launch();
    }
}
