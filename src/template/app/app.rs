pub mod fn_app_template {
    // --------------- //
    use uuid::Uuid as uid;
    use rocket::Data;
    use csv::{ Reader, StringRecord, Error };
    use mongodb::Collection;
    use crate::template::model::models::mods_template::Template;
    use crate::helpers::read::csv::read_csv::fn_read_csv;
    use crate::template::data::data::data_template_fn;
    // --------------- //

    pub fn upload (data: Data, collection: &Collection) {
        let file_id             = uid::new_v4();
        let file_id_str: &str   = &format!("/tmp/{}.csv", file_id.to_string());
        let _                   = data.stream_to_file(file_id_str);
        let read                = Reader::from_path(file_id_str).unwrap();
        let rows: Vec<Template> = Vec::new();
        let func                = |value: StringRecord, header: StringRecord| -> Result<Template, Error> {
            value.deserialize::<Template>(Some(&header))
        };
        let response            = fn_read_csv(read, rows, func);
        for item in response.0 {
            data_template_fn::create(item, collection);
        }
    }
}
