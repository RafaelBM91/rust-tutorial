pub mod fn_app_template {
    // --------------- //
    use uuid::Uuid as uid;
    use rocket::Data;
    use csv::{ Reader, StringRecord, Error };
    use mongodb::Collection;
    use crate::template::model::models::mods_template::Template;
    use crate::template::data::model::dt_template::DTemplate;
    use crate::template::data::data::data_template_fn;
    use crate::helpers::read::csv::read_csv::fn_read_csv;
    use crate::helpers::models::csv::csv::mods_csv::ErrorLoadDetail;
    // --------------- //

    pub fn upload (data: Data, collection: &Collection) -> (Vec<DTemplate>, Vec<Option<ErrorLoadDetail>>) {
        let file_id                      = uid::new_v4();
        let file_id_str: &str            = &format!("/tmp/{}.csv", file_id.to_string());
        let _                            = data.stream_to_file(file_id_str);
        let read                         = Reader::from_path(file_id_str).unwrap();
        let rows: Vec<Template>          = Vec::new();
        let mut inserted: Vec<DTemplate> = Vec::new();
        let func                         = |value: StringRecord, header: StringRecord| -> Result<Template, Error> {
            value.deserialize::<Template>(Some(&header))
        };
        let response            = fn_read_csv(read, rows, func);
        for item in response.0 {
            inserted.push(data_template_fn::create(item, collection));
        }
        (inserted, response.1)
    }

    pub fn find (collection: &Collection) -> Vec<DTemplate> {
        data_template_fn::find(collection)
    }

}
