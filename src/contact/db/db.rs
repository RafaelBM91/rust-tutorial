pub mod db {
    
    use crate::diesel::{ insert_into, RunQueryDsl };
    use crate::contact::models::create::mods_contact::{
        Contact,
        PersonalInfo,
        CompanyInfo
    };
    use crate::connection::connection::connection::connect;
    use crate::contact::schema::{
        contacts,
        personals,
        companies
    };

    pub fn insert_contact (data: Contact) -> Contact {
        insert_into(contacts::table)
            .values( &data )
            .get_result::<Contact>( &connect() )
            .expect("Error saving new contact")
    }

    pub fn insert_personal (data: PersonalInfo) -> PersonalInfo {
        insert_into(personals::table)
            .values( &data )
            .get_result::<PersonalInfo>( &connect() )
            .expect("Error saving new personal info")
    }

    pub fn insert_company (data: CompanyInfo) -> CompanyInfo {
        insert_into(companies::table)
            .values( &data )
            .get_result::<CompanyInfo>( &connect() )
            .expect("Error saving new company info")
    }

}
