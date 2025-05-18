use std::error::Error;

use crate::log;

use crate::schema::users::dsl::*;
use crate::schema::users::{self};
use crate::statics::DB_POOL;
use ::password_hash::rand_core::OsRng;
use actix_session::Session;
use actix_web::web::get;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use diesel::RunQueryDsl;
use diesel::prelude::{Insertable, Queryable};
use diesel::query_dsl::methods::*;
use diesel::{ExpressionMethods, QueryResult};

use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id_user: i32,
    pub mail: String,
    pub phone_number: String,
    pub password_hash: String,
    pub date_creation: String, // YYYY-MM-DD HH:MM:SS
    pub admin: i32,
}

impl User {
    /// Compute the hash of the password and check if the mail/password match in the DB return None if not match.
    /// Return Some(n) where n is the ID if the user exist
    pub fn verify_login(
        mail_input: &str,
        password_input: &str,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let mut con = DB_POOL.get()?;

        // Try to find the user by email
        let user = match users.filter(mail.eq(mail_input)).first::<User>(&mut con) {
            Ok(user) => user,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(Box::new(e)),
        };

        // Parse password hash
        let hashed_password = PasswordHash::new(&user.password_hash)
            .map_err(|e| Box::<dyn Error>::from(e.to_string()))?;

        // Verify password
        match Argon2::default().verify_password(password_input.as_bytes(), &hashed_password) {
            Ok(_) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
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
            .order(id_user.desc())
            .first::<User>(&mut conn)
    }
    /// Recherche par id.
    /// - `Ok(Some(user))` si trouvé
    /// - `Ok(None)`      si l'id n'existe pas
    /// - `Err(e)`        pour toute autre erreur (connexion, etc.)
    pub fn get(id: i32) -> Result<Option<Self>, Box<dyn Error>> {
        let mut conn = DB_POOL.get()?;

        match users.find(id).first::<Self>(&mut conn) {
            Ok(user) => Ok(Some(user)),

            // Cas « pas trouvé » → Ok(None)
            Err(DieselError::NotFound) => Ok(None),

            // Toute autre erreur → Err(e)
            Err(e) => {
                log!("error when searching user {id}: {e}");
                Err(Box::new(e))
            }
        }
    }

    /// Récupère l'utilisateur présent dans la session.
    /// - `Ok(Some(user))` si id_user présent et trouvé
    /// - `Ok(None)`      si pas d'id_user ou id inexistant
    /// - `Err(e)`        si erreur d'accès session ou DB hors NotFound
    pub fn from_session(session: &Session) -> Result<Option<Self>, Box<dyn Error>> {
        match session.get::<i32>("id_user") {
            // id_user présent → on tente le get
            Ok(Some(uid)) => Self::get(uid),

            // pas d'id_user en session → None
            Ok(None) => Ok(None),

            // erreur d'accès à la session
            Err(e) => {
                log!("error when accessing session id_user: {e}");
                Err(Box::new(e))
            }
        }
    }
}
