use serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::schema::classes;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Class {
    pub id: i32,
    pub code: String,
    pub url: String,
    pub questions: Vec<Option<String>>,
    pub upvotes: Vec<Option<i32>>,
}

#[derive(Insertable)]
#[table_name = "classes"]
pub struct NewClass {
    pub code: String,
    pub url: String,
    pub questions: Vec<Option<String>>,
    pub upvotes: Vec<Option<i32>>,
}

#[derive(Serialize, Deserialize)]
pub struct ClassPayload {
    pub code: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionPayload {
    pub question: String,
}
