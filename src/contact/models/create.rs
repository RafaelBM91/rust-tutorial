pub mod mods_contact {

    use crate::contact::schema::{
        contacts,
        personals,
        companies
    };

    #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
    #[table_name = "contacts"]
    pub struct Contact {
        pub id       : String,
        pub name     : String,
        pub last_name: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct PersonalInfoRequest {
        pub id        : Option<String>,
        pub contact_id: Option<String>,
        pub cellphone : String,
        pub email     : Option<String>,
        pub address   : Option<String>,
    }

    impl PersonalInfoRequest {
        pub fn hydrate (cellphones: Vec<String>, emails: Vec<String>, address: Vec<String>) -> Vec<PersonalInfoRequest> {

            let rows: Vec<PersonalInfoRequest> = Vec::new();
            let cont = 0;
            
            for item in cellphones {

                emails[cont]

                cont += 1;
            }

        }
    }

    #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
    #[table_name = "personals"]
    pub struct PersonalInfo {
        pub id        : String,
        pub contact_id: String,
        pub cellphone : String,
        pub email     : String,
        pub address   : String,
    }

    impl PersonalInfo {
        pub fn to_save (data: PersonalInfoRequest, id: String, contact_id: String) -> PersonalInfo {
            PersonalInfo {
                id        : id,
                contact_id: contact_id,
                cellphone : data.cellphone,
                email     : data.email.unwrap_or_default(),
                address   : data.address.unwrap_or_default(),
            }
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct CompanyInfoRequest {
        pub id         : Option<String>,
        pub contact_id : Option<String>,
        pub phone      : String,
        pub email      : Option<String>,
        pub address    : Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
    #[table_name = "companies"]
    pub struct CompanyInfo {
        pub id         : String,
        pub contact_id : String,
        pub phone      : String,
        pub email      : String,
        pub address    : String,
    }

    impl CompanyInfo {
        pub fn to_save (data: CompanyInfoRequest, id: String, contact_id: String) -> CompanyInfo {
            CompanyInfo {
                id        : id,
                contact_id: contact_id,
                phone     : data.phone,
                email     : data.email.unwrap_or_default(),
                address   : data.address.unwrap_or_default(),
            }
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct CreateContactRequest {
        pub id           : Option<String>,
        pub name         : String,
        pub last_name    : String,
        pub personal_data: Vec<PersonalInfoRequest>,
        pub company_data : Option<Vec<CompanyInfoRequest>>,
    }

    impl CreateContactRequest {
        pub fn extract_contact (contact: CreateContactRequest, id: String) -> Contact {
            Contact {
                id       : id,
                name     : contact.name,
                last_name: contact.last_name,
            }
        }
    }

}
