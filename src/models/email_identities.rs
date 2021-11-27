use crate::schema::email_identities;
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable, Associations};
use super::users::User;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
#[table_name="email_identities"]
pub struct EmailIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub hash: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Associations,  Debug, PartialEq)]
#[belongs_to(User)]
#[table_name="email_identities"]
pub struct NewEmailIdentity<'a > {
    pub id: Option<Uuid>,
    pub user_id: &'a Uuid,
    pub hash: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl<'a > NewEmailIdentity<'a > {
    pub fn new(user_id: &'a Uuid, hash: String, email: &str) -> NewEmailIdentity<'a > {
        NewEmailIdentity {
            id: None,
            user_id,
            hash,
            email: String::from(email),
            created_at: None,
            updated_at: None,
        }
    }
}