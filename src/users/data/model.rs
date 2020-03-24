pub mod dt_users {
    // --------------- //
    use bson::oid::ObjectId;
    // --------------- //

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct DUsers {
        #[serde(rename = "_id")]
        pub id       : ObjectId,
        pub email    : String,
        pub name     : String,
        pub last_name: String,
        pub company  : String,
        pub password : String,
    }
    impl DUsers {
        pub fn clear_password (&mut self) {
            self.password = String::from("");
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct DLogIn {
        #[serde(rename = "_id")]
        pub id  : String,
        pub user: DUsers,
    }
}
