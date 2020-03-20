pub mod read_csv {

    // --------------- //
    use csv::{
        Reader,
        StringRecord,
        Error,
        ErrorKind::Deserialize as EKDeserialize
    };
    use std::fs::File;
    use crate::helpers::models::csv::csv::mods_csv::ErrorLoadDetail;
    // --------------- //

    pub fn fn_read_csv <T, F> (
        mut read: Reader<File>,
        mut rows: Vec<T>,
        func: F
    ) -> (Vec<T>, Vec<Option<ErrorLoadDetail>>)
    where F: Fn(StringRecord, StringRecord) -> Result<T, Error> {
        let mut errors: Vec<Option<ErrorLoadDetail>> = Vec::new();
        let mut header: StringRecord = StringRecord::new();

        match read.headers() {
            Ok(headers) => header = headers.clone(),
            Err(_) => (),
        }

        for result in read.records() {
            match result {
                Ok(value) => match func(value, header.clone()) {
                    Ok(value) => rows.push(value),
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

    fn catch_error(error: Error) -> Option<ErrorLoadDetail> {
        match error.kind() {
            EKDeserialize {
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
} 