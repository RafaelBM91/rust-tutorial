pub mod dt_template {
    // --------------- //
    use bson::{
        oid::ObjectId,
        ordered::OrderedDocument,
    };
    use crate::helpers::{
        models::csv::csv::mods_csv::SpecialDateTime,
        utils::parse_ordered::ordered::{
            resolve_attribute_filter_array_string,
            resolve_attribute_sort,
        },
    };
    // --------------- //

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct DTemplate {
        #[serde(rename = "_id")]
        pub id                    : ObjectId,
        pub funnel_status         : String,
        pub decision_maker        : String,
        pub last_name             : String,
        pub company_area          : String,
        pub company_position      : String,
        pub personal_mail         : String,
        pub personal_companymail  : String,
        pub cellphone_whatsapp    : String,
        pub skype_user            : String,
        pub hangout_user          : String,
        pub linkedin_url          : String,
        pub picture_url           : String,
        pub facebook_url          : String,
        pub instagram_url         : String,
        pub interests             : Vec<String>,
        pub sex                   : String,
        pub nse                   : String,
        pub birthday              : SpecialDateTime,
        pub media_consumption     : Vec<String>,
        pub company_linkedin_url  : String,
        pub company_name          : String,
        pub potential_size        : String,
        pub company_sector        : String,
        pub company_products      : Vec<String>,
        pub web_url               : String,
        pub company_phone         : String,
        pub sucursal_location     : String,
        pub city                  : String,
        pub state                 : String,
        pub country               : String,
        pub nextpurchase_date     : String,
        pub satisfaction_dm       : String,
        pub operator_mailid       : String,
        pub countable_number      : String,
        pub dm_countable          : String,
        pub personal_countablemail: String,
        pub cellphone_countable   : String,
        pub payment_date          : SpecialDateTime,
        pub frecuency             : String,
        pub payment_method        : String,
        pub payment_ammount       : f32,
        pub status_countable      : String,
        pub payment_description   : String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct FindTemplateFilters {
        pub name          : Option<Vec<String>>,
        pub funnel_status : Option<Vec<String>>,
        pub company_sector: Option<Vec<String>>,
        pub potential_size: Option<Vec<String>>,
        pub country       : Option<Vec<String>>,
    }
    impl FindTemplateFilters {
        pub fn apply_filters (self) -> OrderedDocument {
            let name = self.name.unwrap_or_default().join("|");
            doc! {
                "$and": [
                    { "$or": [
                        { "decision_maker": {
                            "$regex"  : format!(".*{}.*", name),
                            "$options": "i",
                        } }, 
                        { "last_name": {
                            "$regex"  : format!(".*{}.*", name),
                            "$options": "i",
                        } }
                    ] },
                    resolve_attribute_filter_array_string("funnel_status", self.funnel_status),
                    resolve_attribute_filter_array_string("company_sector", self.company_sector),
                    resolve_attribute_filter_array_string("potential_size", self.potential_size),
                    resolve_attribute_filter_array_string("country", self.country),
                ]
            }
        }
    }

    #[derive(Debug, FromFormValue, Serialize, Deserialize, Clone)]
    pub enum SortTemplateResultValues {
        Asc,
        Desc,
    }
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct SortTemplateResult {
        pub company_sector: Option<SortTemplateResultValues>
    }
    impl SortTemplateResult {
        pub fn apply_sort (self) -> OrderedDocument {
            let mut attributes = OrderedDocument::new();
            resolve_attribute_sort(&mut attributes, "company_sector", self.company_sector);
            attributes
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct FindTemplateParams {
        pub _limit  : i64,
        pub _filters: FindTemplateFilters,
        pub _sort   : SortTemplateResult
    }
}
