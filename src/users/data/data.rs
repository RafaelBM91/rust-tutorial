pub mod data_users_fn {
    // --------------- //
    use mongodb::{
        Collection,
    };
    use crate::users::model::models::mods_users::{
        User,
        LogIn,
    };
    use crate::users::data::model::dt_users::DUsers;
    use crate::helpers::utils::parse_ordered::ordered::struct_to_ordered;
    // --------------- //

    pub fn create (user: User, collection: &Collection) -> DUsers {
        let user_insert = user.parse();
        let document = struct_to_ordered(user_insert.clone());
        match collection.insert_one(document, None) {
            Ok(_)  => (),
            Err(_) => println!("[error Insert to Doc]")
        }
        user_insert
    }

    pub fn longin (login: LogIn, collection: &Collection) -> Option<DUsers> {
        let document = struct_to_ordered(login);
        match collection.find_one(document, None) {
            Ok(value) => {
                if let Some(user) = value {
                    let d_user: DUsers = bson::from_bson(
                        bson::Bson::Document(user)
                    ).unwrap();
                    return Some(d_user);
                }
                return None;
            },
            Err(_)    => None,
        }
    }
}
