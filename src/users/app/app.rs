pub mod fn_app_users {
    // --------------- //
    use mongodb::Collection;
    use crate::users::{
        model::models::mods_users::{
            User,
            LogIn,
        },
        data::model::dt_users::{
            DUsers,
            DLogIn,
        },
    };
    use crate::users::data::data::data_users_fn;
    use crate::authorization::auth::authorization::Lock;
    // --------------- //

    pub fn create (user: User, collection: &Collection) -> DUsers {
        let mut d_users = data_users_fn::create(user, collection);
        d_users.clear_password();
        d_users
    }

    pub fn login (mut login: LogIn, collection: &Collection) -> Option<DLogIn> {
        login = login.crypt_password();
        if let Some(mut d_user) = data_users_fn::longin(login, collection) {
            d_user.clear_password();
            if let Some(session) = Lock::create_session(d_user.id.to_hex()) {
                let d_login = DLogIn {
                    id  : session,
                    user: d_user,
                };
                return Some(d_login);
            }
        }
        None
    }
}
