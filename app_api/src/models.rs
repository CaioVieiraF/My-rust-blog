use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i32,
}
