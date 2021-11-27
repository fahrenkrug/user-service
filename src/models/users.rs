use crate::schema::users;
use diesel::{Queryable, Insertable, Identifiable};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq)]
#[table_name="users"]
pub struct NewUser<'a > {
    pub id: Option<Uuid>,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
