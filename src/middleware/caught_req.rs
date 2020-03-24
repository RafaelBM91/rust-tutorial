pub mod caught {
    // --------------- //
    use rocket::{
        http::Header,
        Data,
        Request,
    };
    use crate::authorization::auth::authorization::Claims;
    // --------------- //

    pub fn caught_authorization (req: &mut Request, _data: &Data) {
        let auth = req.headers().get_one("Authorization");
        match auth {
            Some(auth_val) => {
                if let Some(value) = Claims::authorize_session(auth_val.to_string()) {
                    req.add_header(
                        Header::new("X-Authorization-Next", value)
                    );
                }
            },
            None => (),
        };
    }
}
