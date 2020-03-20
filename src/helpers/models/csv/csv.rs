pub mod mods_csv {
    // --------------- //
    use bson::UtcDateTime;
    // --------------- //

    #[derive(Debug, Serialize, Clone)]
    pub struct ErrorLoadDetail {
        pub line   : u64,
        pub cell   : u64,
        pub message: String,
    }

    #[derive(Serialize)]
    pub struct CustomResponse {
        pub message : String,
        pub inserted: u64,
        pub fail    : Vec<Option<ErrorLoadDetail>>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SpecialDateTime {
        pub value_utc: UtcDateTime,
        pub value_str: String,
    }
}
