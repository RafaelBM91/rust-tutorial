pub mod mods_users {
    // --------------- //
    use bson::oid::ObjectId;
    use crate::users::data::model::dt_users::DUsers;
    use crate::helpers::utils::crypto_string::crypto_fn::crypt_string_sha256;
    // --------------- //

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct User {
        pub email    : String,
        pub name     : String,
        pub last_name: String,
        pub company  : String,
        pub password : String,
    }
    impl User {
        pub fn parse (self) -> DUsers {
            DUsers {
                id       : ObjectId::new().unwrap(),
                email    : self.email.clone(),
                name     : self.name.clone(),
                last_name: self.last_name.clone(),
                company  : self.company.clone(),
                password : crypt_string_sha256(self.password),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct LogIn {
        pub email    : String,
        pub password : String,
    }
    impl LogIn {
        pub fn crypt_password (self) -> LogIn {
            LogIn {
                email   : self.email,
                password: crypt_string_sha256(self.password),
            }
        }
    }
}
