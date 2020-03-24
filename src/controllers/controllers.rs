pub mod controller {
    // --------------- //
    use rocket::{
        http::Status,
        State,
        Data,
        response::status::{
            Custom,
            BadRequest,
        },
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
    use crate::middleware::intercept::interceptor::AuthCaught;
    // --------------- //

    #[post("/upload", format="application/octet-stream", data="<data>")]
    fn upload (data: Data, state: State<Vec<Collection>>) -> Custom<Json<CustomResponse>> {
        let response = fn_app_template::upload(data, &state.inner()[0]);
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
        state : State<Vec<Collection>>
    ) -> Json<Vec<DTemplate>> {
        Json(fn_app_template::find(params.into_inner(), &state.inner()[0]))
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Intercept {
        another: String
    }

    #[post("/intercept", format="application/json", data="<_params>")]
    fn intercept (
        _params: Json<Intercept>,
        _id: AuthCaught,
        state : State<Vec<Collection>>,
    ) -> &'static str {
        println!("Result: {:?}", &state.inner()[0]);
        "Sensitive data."
    }

    use crate::users::app::app::fn_app_users;
    use crate::users::model::models::mods_users::{
        User,
        LogIn,
    };
    use crate::users::data::model::dt_users::{
        DUsers,
        DLogIn,
    };

    #[post("/create", format="application/json", data="<params>")]
    fn create_user (
        params: Json<User>,
        state : State<Vec<Collection>>,
    ) -> Json<DUsers> {
        Json(fn_app_users::create(
            params.into_inner(),
            &state.inner()[1],
        ))
    }

    #[post("/login", format="application/json", data="<params>")]
    fn login_user (
        params: Json<LogIn>,
        state : State<Vec<Collection>>,
    ) -> Result<Json<DLogIn>, BadRequest<&'static str>> {
        match fn_app_users::login(
            params.into_inner(),
            &state.inner()[1],
        ) {
            Some(value) => Ok(Json(value)),
            None        => Err(BadRequest(Some("Error login, email or password fail."))),
        }
    }

    pub fn engine () {
        let client  : Client     = connect();
        let database: Database   = client.database("test");
        let template: Collection = database.collection("templates");
        let user    : Collection = database.collection("users");
        let collections: Vec<Collection> = vec![template, user];

        rocket::ignite()
            .manage(collections)
            .mount("/", StaticFiles::from("view"))
            .mount("/api", routes![upload,find,intercept])
            .mount("/user", routes![create_user,login_user])
            .launch();
    }
}
