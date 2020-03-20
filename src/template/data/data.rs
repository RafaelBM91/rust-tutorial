pub mod data_template_fn {
    // --------------- //
    use crate::template::model::models::mods_template::Template;
    use crate::template::data::model::dt_template::DTemplate;
    use mongodb::Collection;
    use bson::Bson::Document;
    use mongodb::options::FindOptions;
    // --------------- //

    pub fn create (template: Template, collection: &Collection) -> DTemplate {
        let data_template = template.parse();
        let serialized_template   = bson::to_bson(&data_template.clone()).unwrap();
        if let Document(document) = serialized_template {
            collection.insert_one(document, None).unwrap();
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }
        data_template
    }

    pub fn find (collection: &Collection) -> Vec<DTemplate> {
        let mut rows: Vec<DTemplate> = Vec::new();
        let options = FindOptions::builder()
            .limit(1)
            .build();
        let cursor = collection.find(doc! {}, options).unwrap();
        for result in cursor {
            if let Ok(item) = result {
                let row: DTemplate = bson::from_bson(
                    bson::Bson::Document(item)
                ).unwrap();
                rows.push(row);
            }
        }
        rows
    }
}
