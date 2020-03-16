table! {
    contacts (id) {
        id        -> VarChar,
        name      -> VarChar,
        last_name -> VarChar,
    }
}

table! {
    personals (id) {
        id         -> VarChar,
        contact_id -> VarChar,
        cellphone  -> VarChar,
        email      -> VarChar,
        address    -> VarChar,
    }
}

table! {
    companies (id) {
        id         -> VarChar,
        contact_id -> VarChar,
        phone  -> VarChar,
        email      -> VarChar,
        address    -> VarChar,
    }
}
