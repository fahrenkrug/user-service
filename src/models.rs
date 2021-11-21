use diesel::Queryable;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}