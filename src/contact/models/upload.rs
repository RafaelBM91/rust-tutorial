pub mod mods_upload {

    use crate::contact::models::create::mods_contact::{
        CreateContactRequest
    };

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FromCsvToModel {
        pub name              : String,
        pub last_name         : String,
        pub personal_cellphone: String,
        pub personal_email    : String,
        pub personal_address  : String,
        pub company_phone     : String,
        pub company_email     : String,
        pub company_address   : String
    }

    impl FromCsvToModel {
        pub fn convert (object: FromCsvToModel) -> CreateContactRequest {
            CreateContactRequest {
                id: None,
                name
            }
        }
        fn string_to_array (value: String) -> Vec<String> {
            value
                .trim()
                .split(',')
                .map(String::from)
                .collect::<Vec<String>>()
        }
    }

    /*
        #[derive(Debug, Deserialize, Clone)]
        pub struct CreateContactRequest {
            pub id           : Option<String>,
            pub name         : String,
            pub last_name    : String,
            pub personal_data: Vec<PersonalInfoRequest>,
            pub company_data : Option<Vec<CompanyInfoRequest>>,
        }
    */

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FromCsvToModelTransition {
        pub name              : String,
        pub last_name         : String,
        pub personal_cellphone: Vec<String>,
        pub personal_email    : Vec<String>,
        pub personal_address  : Vec<String>,
        pub company_phone     : Vec<String>,
        pub company_email     : Vec<String>,
        pub company_address   : Vec<String>,
    }

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
}
