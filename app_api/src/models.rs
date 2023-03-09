use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::posts;

#[derive(Queryable, Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: i32,
}

#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: i32,
}
