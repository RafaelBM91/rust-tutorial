pub mod application {

    use uuid::Uuid as uid;
    use rocket::Data;
    use csv::Reader;
    use crate::contact::models::create::mods_contact::{
        CreateContactRequest,
        Contact,
        PersonalInfo,
        CompanyInfo
    };
    use crate::contact::db::db::db::{
        insert_contact,
        insert_personal,
        insert_company
    };
    use crate::contact::models::upload::mods_upload::{
        CustomResponse
    };
    use crate::contact::helpers::read_csv::helpers_read_csv::read_csv;

    pub fn fn_create_contact (data: CreateContactRequest) -> Contact {
        let personal_data = data.personal_data.clone();
        let company_data  = data.company_data.clone().unwrap_or_default();
        let contact       = insert_contact(
            CreateContactRequest::extract_contact(
                data,
                uid::new_v4().to_string(),
            )
        );
        for item in personal_data {
            let personal_info = PersonalInfo::to_save(
                item,
                uid::new_v4().to_string(),
                contact.id.clone(),
            );
            insert_personal(personal_info);
        }
        for item in company_data {
            let company_info = CompanyInfo::to_save(
                item,
                uid::new_v4().to_string(),
                contact.id.clone(),
            );
            insert_company(company_info);
        }
        contact
    }

    pub fn fn_upload_contact (data: Data) -> CustomResponse {
        let file_id = uid::new_v4();
        let file_id_str: &str = &format!("/tmp/{}.csv", file_id.to_string());
        let _stream = data.stream_to_file(file_id_str);
        let read = Reader::from_path(file_id_str).unwrap();

        let (rows, errors) = read_csv(read);

        CustomResponse {
            message: format!("elements inserts {}", rows.len()),
            inserted: rows.len() as u64,
            fail: errors,
        }
    }
}
