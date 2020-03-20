pub mod data_template_fn {
    // --------------- //
    use crate::template::model::models::mods_template::Template;
    use mongodb::Collection;
    use bson::Bson::Document;
    // --------------- //

    pub fn create (template: Template, collection: &Collection) {
        let serialized_template   = bson::to_bson(&template.parse()).unwrap();
        if let Document(document) = serialized_template {
            let result = collection.insert_one(document, None).unwrap();
            println!("{:?}", result);
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }

    }
}
