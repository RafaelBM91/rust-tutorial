pub mod routes_fn {
    // --------------- //
    use rocket::{
        State,
        Route,
        response::status::BadRequest,
    };
    use rocket_contrib::json::Json;
    use mongodb::Collection;
    use crate::users::app::app::fn_app_users;
    use crate::users::model::models::mods_users::{
        User,
        LogIn,
    };
    use crate::users::data::model::dt_users::{
        DUsers,
        DLogIn,
    };
    // --------------- //

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

    pub fn user_routes_fn () -> Vec<Route> {
        routes![create_user, login_user]
    }
}
