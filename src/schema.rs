use diesel::Queryable;
use serde::{Deserialize,Serialize};


diesel::table!{
    use diesel::sql_types::*;
    #[allow(non_snake_case)]
    BlogEntries(id) {
        id -> Integer,
        heading -> VarChar,
        url -> VarChar,
        content -> Text,
        author -> Nullable<VarChar>,
    }
}

#[derive(Queryable,Debug)]
#[derive(Serialize, Deserialize)]
pub struct BlogEntry{
    pub id: i32,
    pub heading: String,
    pub url: String,
    pub content: String,
    pub author: Option<String>
}