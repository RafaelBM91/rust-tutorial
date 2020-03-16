pub mod helpers_read_csv {
    use csv::Reader;
    use std::fs::File;
    use crate::contact::models::{
        create::mods_contact::{
            CreateContactRequest
        },
        upload::mods_upload::{
            ErrorLoadDetail,
            FromCsvToModel,
            FromCsvToModelTransition,
        }
    };

    pub fn read_csv(mut read: Reader<File>) -> (Vec<CreateContactRequest>, Vec<Option<ErrorLoadDetail>>) {
        let mut rows: Vec<CreateContactRequest> = Vec::new();
        let mut errors: Vec<Option<ErrorLoadDetail>> = Vec::new();
        let mut header: csv::StringRecord = csv::StringRecord::new();

        match read.headers() {
            Ok(headers) => header = headers.clone(),
            Err(_) => (),
        }

        for result in read.records() {

            match result {
                Ok(value) => match value.deserialize::<FromCsvToModel>(Some(&header)) {
                    Ok(value) => {
                        
                        // let mut row: CreateContactRequest = ;

                        let t = FromCsvToModelTransition {
                            name              : value.name,
                            last_name         : value.last_name,
                            personal_cellphone: string_to_array(value.personal_cellphone),
                            personal_email    : string_to_array(value.personal_email),
                            personal_address  : string_to_array(value.personal_address),
                            company_phone     : string_to_array(value.company_phone),
                            company_email     : string_to_array(value.company_email),
                            company_address   : string_to_array(value.company_address),
                        };

                        println!("{:?}", t);
                    },
                    Err(err) => {
                        let error: ErrorLoadDetail = catch_error(err).unwrap();
                        errors.push(Some(error.clone()));
                        if error.line == 0 {
                            break;
                        }
                    }
                },
                Err(_) => (),
            }
        }
        (rows, errors)
    }

    fn catch_error(error: csv::Error) -> Option<ErrorLoadDetail> {
        match error.kind() {
            csv::ErrorKind::Deserialize {
                pos: _pos,
                err: _err,
            } => {
                let position = _pos.as_ref().unwrap();
                let line = if _err.field().is_some() {
                    position.line()
                } else {
                    0
                };
                let cell = if _err.field().is_some() {
                    _err.field().unwrap() + 1
                } else {
                    0
                };
                let message = _err.kind().to_string();
                Some(ErrorLoadDetail {
                    line,
                    cell,
                    message,
                })
            }
            _ => None,
        }
    }

    fn string_to_array (value: String) -> Vec<String> {
        value
            .trim()
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>()
    }
}
