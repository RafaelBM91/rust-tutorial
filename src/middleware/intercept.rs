pub mod interceptor {
    // --------------- //
    use rocket::Outcome;
    use rocket::http::Status;
    use rocket::request::{self, Request, FromRequest};
    use crate::authorization::auth::authorization::Lock;
    // --------------- //

    #[derive(Debug)]
    pub struct AuthCaught(pub String);

    fn is_valid(key: &str) -> bool {
        match Lock::authorize_session(key.to_string()) {
            Some(_) => true,
            None    => false,
        }
    }

    #[derive(Debug)]
    pub enum AuthCaughtError {
        BadCount,
        Missing,
        Invalid,
    }

    impl<'a, 'r> FromRequest<'a, 'r> for AuthCaught {
        type Error = AuthCaughtError;
        fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
            let keys: Vec<_> = request.headers().get("Authorization").collect();
            match keys.len() {
                0 => Outcome::Failure((Status::BadRequest, AuthCaughtError::Missing)),
                1 if is_valid(keys[0]) => {
                    let id = Lock::authorize_session(keys[0].to_string()).unwrap();
                    return Outcome::Success(AuthCaught(id));
                },
                1 => Outcome::Failure((Status::BadRequest, AuthCaughtError::Invalid)),
                _ => Outcome::Failure((Status::BadRequest, AuthCaughtError::BadCount)),
            }
        }
    }
}
