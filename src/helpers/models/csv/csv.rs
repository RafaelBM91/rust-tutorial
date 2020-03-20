pub mod mods_csv {
    #[derive(Debug, Serialize, Clone)]
    pub struct ErrorLoadDetail {
        pub line   : u64,
        pub cell   : u64,
        pub message: String,
    }
}
