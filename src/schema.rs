use diesel::Queryable;

diesel::table!{
    use diesel::sql_types::*;
    blog_entrys(id) {
        id -> Integer,
        heading -> VarChar,
        url -> VarChar,
        content -> Text,
        author -> Nullable<VarChar>,
    }
}

#[derive(Queryable,Debug)]
pub struct BlogEntry{
    pub id: i32,
    pub heading: String,
    pub url: String,
    pub content: String,
    pub author: Option<String>
}