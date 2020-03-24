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
    pub struct Claims {
        pub id : String,
        pub exp: i64,
    }
    impl Claims {
        pub fn create_session (id: String) -> Option<String> {
            let at_moment = offset::Utc::now();
            let dtu       = at_moment
                .checked_add_signed(
                    Duration::seconds(5)
                ).unwrap();
    
            let my_claims = Claims {
                id : id,
                exp: dtu.timestamp(),
            };
    
            match encode(
                &Header::default(),
                &my_claims,
                &EncodingKey::from_secret("secret".as_ref())
            ) {
                Ok(token) => Some(token),
                Err(_)    => None,
            }
        }
        pub fn authorize_session (token: String) -> Option<String> {
            match decode::<Claims>(
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
