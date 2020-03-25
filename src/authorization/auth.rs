pub mod authorization {
    // --------------- //
    use jsonwebtoken::{
        encode,
        decode,
        Header,
        Validation,
        EncodingKey,
        DecodingKey,
    };
    use chrono::{
        offset,
        Duration,
    };
    // --------------- //

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Lock {
        pub id : String,
        pub exp: i64,
    }
    impl Lock {
        pub fn create_session (id: String) -> Option<String> {
            let at_moment = offset::Utc::now();
            let dtu       = at_moment
                .checked_add_signed(
                    // Duration::seconds(60)
                    Duration::days(30)
                ).unwrap();
    
            let my_lock = Lock {
                id : id,
                exp: dtu.timestamp(),
            };
    
            match encode(
                &Header::default(),
                &my_lock,
                &EncodingKey::from_secret("secret".as_ref())
            ) {
                Ok(token) => Some(token),
                Err(_)    => None,
            }
        }
        pub fn authorize_session (token: String) -> Option<String> {
            match decode::<Lock>(
                &token,
                &DecodingKey::from_secret(
                    "secret".as_ref()
                ),
                &Validation::default()
            ) {
                Ok(value) => Some(value.claims.id),
                Err(er)   => {
                    println!("{:?}", er);
                    return None;
                },
            }
        }
    }    
}
