pub mod routes_fn {
    // --------------- //
    use rocket::{
        http::Status,
        State,
        Data,
        Route,
        response::status::Custom,
    };
    use rocket_contrib::json::Json;
    use mongodb::Collection;
    use crate::template::app::app::fn_app_template;
    use crate::template::data::model::dt_template::DTemplate;
    use crate::template::data::model::dt_template::FindTemplateParams;
    use crate::helpers::models::csv::csv::mods_csv::CustomResponse;
    use crate::middleware::intercept::interceptor::AuthCaught;
    // --------------- //

    #[post("/upload", format="application/octet-stream", data="<data>")]
    fn upload_template (
        data : Data,
        state: State<Vec<Collection>>,
        id   : AuthCaught,
    ) -> Custom<Json<CustomResponse>>
    {
        println!("{:?}", id );
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
    fn find_template (
        params: Json<FindTemplateParams>,
        state : State<Vec<Collection>>,
        id    : AuthCaught,
    ) -> Json<Vec<DTemplate>> {
        println!("{:?}", id.0 );
        Json(fn_app_template::find(params.into_inner(), &state.inner()[0]))
    }

    pub fn template_routes_fn () -> Vec<Route> {
        routes![upload_template, find_template]
    }
}
