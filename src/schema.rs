// table! {
//     models {
//         id -> Text,
//         name -> Text,
//         age -> Integer,
//     }
// }

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}
