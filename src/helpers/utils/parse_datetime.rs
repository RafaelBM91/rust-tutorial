pub mod date_time {
    // --------------- //
    use chrono::{ DateTime, NaiveDateTime, Utc };
    use bson::UtcDateTime;
    use crate::helpers::models::csv::csv::mods_csv::SpecialDateTime;
    // --------------- //

    pub fn date_time_from_string (mut date_time: String) -> SpecialDateTime {
        
        if date_time.len() < 10 { 
            date_time = format!("{} 00:00:00", date_time);
        }

        let tup_resp = parse_native_dt(date_time.clone());

        let dt = DateTime::<Utc>::from_utc(
            tup_resp.1,
            Utc
        );

        SpecialDateTime {
            value_utc: UtcDateTime::from(dt),
            value_str: tup_resp.0,
        }
    }

    fn parse_native_dt (dt: String) -> (String, NaiveDateTime) {
        match NaiveDateTime::parse_from_str( &dt.clone() , "%d/%m/%Y %H:%M:%S") {
            Ok(value) => (dt, value),
            Err(_)    => {
                return (
                    "00/00/0000 00:00:00".to_string(),
                    NaiveDateTime::parse_from_str("01/01/0001 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap()
                )
            }
        }
    }
}
