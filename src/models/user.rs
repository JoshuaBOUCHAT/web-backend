use crate::DB_POOL;

use crate::schema::users::dsl::*;
use crate::schema::users::{self};
use ::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::prelude::{Insertable, Queryable};
use diesel::query_dsl::methods::*;

use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id_users: i32,
    pub mail: String,
    pub phone_number: String,
    pub password_hash: String,
    pub date_creation: String, // YYYY-MM-DD HH:MM:SS
    pub admin: i32,
}

impl User {
    /// Compute the hash of the password and check if the mail/password match in the DB return None if not match.
    /// Return Some(n) where n is the ID if the user exist
    pub fn verfify_login(mail_input: &str, password_input: &str) -> Option<i32> {
        let mut con = DB_POOL.get().ok()?;
        let user = users
            .filter(mail.eq(mail_input))
            .first::<User>(&mut con)
            .ok()?;
        let hashed_pasword = PasswordHash::new(&user.password_hash).ok()?;

        if Argon2::default()
            .verify_password(password_input.as_bytes(), &hashed_pasword)
            .is_ok()
        {
            return Some(user.id_users);
        }
        return None;
    }

    pub fn add_user(
        mail_input: &str,
        phone_input: &str,
        password_input: &str,
    ) -> Result<User, DieselError> {
        use chrono::Utc;
        let mut conn = DB_POOL.get().map_err(|_| DieselError::NotFound)?;

        // Generate salt + hash
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password_input.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Insertable struct (without id as it's auto-incremented)
        #[derive(Insertable)]
        #[diesel(table_name = users)]
        struct NewUser<'a> {
            mail: &'a str,
            phone_number: &'a str,
            password_hash: &'a str,
            date_creation: &'a str,
        }

        let new_user = NewUser {
            mail: mail_input,
            phone_number: phone_input,
            password_hash: &hashed_password,
            date_creation: &Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        eprintln!("build the user try inserting");

        let res = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn);
        println!("Result of insertion:{:?}", res);

        // Return inserted user (simplified way to get back)
        users
            .filter(mail.eq(mail_input))
            .order(id_users.desc())
            .first::<User>(&mut conn)
    }
    pub fn get(id_user: i32) -> Option<Self> {
        let mut conn = DB_POOL.get().ok()?;
        users.find(id_user).first(&mut conn).ok()
    }
}
