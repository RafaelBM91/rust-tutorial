pub mod get_connection {
    // --------------- //
    use dotenv::dotenv;
    use std::env;
    use mongodb::Client;
    // --------------- //

    pub fn connect () -> Client {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = Client::with_uri_str(&mongo_url)
            .expect("Failed to initialize client.");
        client
    }
}
