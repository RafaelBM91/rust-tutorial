pub mod dt_template {
    // --------------- //
    use bson::oid::ObjectId;
    // --------------- //

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DTemplate {
        #[serde(rename = "_id")]
        pub id                  : ObjectId,
        pub funnel_status       : String,
        pub decision_maker      : String,
        pub last_name           : String,
        pub company_area        : String,
        pub company_position    : String,
        pub personal_mail       : String,
        pub personal_companymail: String,
        pub cellphone_whatsapp  : String,
        pub skype_user          : String,
        pub hangout_user        : String,
        pub linkedin_url        : String,
        pub picture_url         : String,
        pub facebook_url        : String,
        pub instagram_url       : String,
        pub interests           : String,
        pub sex                 : String,
        pub nse                 : String,
        pub birthday            : String,
        pub media_consumption   : String,
        pub company_linkedin_url: String,
        pub company_name        : String,
        pub potential_size      : String,
        pub company_sector      : String,
        pub company_products    : String,
        pub web_url             : String,
        pub company_phone       : String,
        pub sucursal_location   : String,
        pub city                : String,
        pub state               : String,
        pub country             : String,
        pub nextpurchase_date   : String,
        pub satisfaction_dm: String,
        pub operator_mailid: String,
        pub countable_number: String,
        pub dm_countable: String,
        pub personal_countablemail: String,
        pub cellphone_countable: String,
        pub payment_date: String,
        pub frecuency: String,
        pub payment_method: String,
        pub payment_ammount: String,
        pub status_countable: String,
        pub payment_description: String,
    }
}
