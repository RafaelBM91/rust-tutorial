pub mod ordered {
    // --------------- //
    use bson::{
        ordered::OrderedDocument,
        Bson::Document,
        to_bson,
    };
    use serde::ser::Serialize;
    use crate::template::data::model::dt_template::SortTemplateResultValues;
    // --------------- //

    pub fn struct_to_ordered <T> (object: T) -> OrderedDocument
        where T: Serialize
    {
        match to_bson(&object) {
            Ok(serialized) => {
                if let Document(document) = serialized {
                    return document;
                } else {
                    return doc! {};
                }
            },
            Err(_)    => {
                return doc! {};
            }
        };
    }

    pub fn resolve_attribute_filter_array_string (key: &str, item: Option<Vec<String>>)
        -> OrderedDocument
    {
        let value = item.unwrap_or_default();
        if value.len() > 0 {
            return doc! {
                key: {
                    "$in": value
                }
            };
        }
        OrderedDocument::new()
    }

    pub fn resolve_attribute_sort (
        attributes: &mut OrderedDocument, key: &str, item: Option<SortTemplateResultValues>
    ) {
        if let Some(unpack) = item {
            match unpack {
                SortTemplateResultValues::Asc => {
                    attributes.insert(key, bson::to_bson(&1).unwrap_or_default())
                },
                SortTemplateResultValues::Desc => {
                    attributes.insert(key, bson::to_bson(&-1).unwrap_or_default())
                }
            };
        }
    }

    // pub fn ordered_not_null_key (ordered_org: OrderedDocument) -> OrderedDocument {
    //     let mut ordered_aux: OrderedDocument = OrderedDocument::new();
    //     let mut keys = ordered_org.keys();
    //     for item in keys.next() {
    //         if !ordered_org.is_null( &item.clone() ) {
    //             ordered_aux.insert( item, ordered_org.get(&item.clone()).unwrap() );
    //         }
    //     }
    //     ordered_aux
    // }
}