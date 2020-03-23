pub mod data_template_fn {
    // --------------- //
    use mongodb::{
        Collection,
        options::FindOptions,
    };
    use crate::template::model::models::mods_template::Template;
    use crate::template::data::model::dt_template::DTemplate;
    use crate::template::data::model::dt_template::FindTemplateParams;
    use crate::helpers::utils::parse_ordered::ordered::struct_to_ordered;
    // --------------- //

    pub fn create (template: Template, collection: &Collection) -> DTemplate {
        let data_template = template.parse();
        let document = struct_to_ordered(data_template.clone());
        match collection.insert_one(document, None) {
            Ok(_)  => (),
            Err(_) => println!("[error Insert to Doc]")
        }
        data_template
    }

    pub fn find (params: FindTemplateParams, collection: &Collection) -> Vec<DTemplate> {
        let mut rows  : Vec<DTemplate> = Vec::new();
        let options                    = FindOptions::builder()
            .limit(params._limit)
            .sort(params._sort.clone().apply_sort())
            .build();
        let filters = params._filters.apply_filters();
        let cursor = collection.find(filters, options).unwrap();
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
