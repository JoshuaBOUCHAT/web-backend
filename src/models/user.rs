use crate::schema::users;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id_users: i32,
    pub mail: String,
    pub phone_number: String,
    pub password_hash: String,
    pub date_creation: String, // YYYY-MM-DD HH:MM:SS
}
