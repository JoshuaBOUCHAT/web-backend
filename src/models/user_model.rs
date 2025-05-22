use std::error::Error;

use crate::utilities::DynResult;
use crate::{log, schema::orders};

use crate::models::order_model::Order;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::statics::DB_POOL;
use ::password_hash::rand_core::OsRng;
use actix_session::Session;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};

use diesel::RunQueryDsl;
use diesel::prelude::{Insertable, Queryable};
use diesel::query_dsl::methods::*;
use diesel::{ExpressionMethods, OptionalExtension};

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
    pub fn from_session_infallible(session: &Session) -> Result<Self, Box<dyn Error>> {
        match session.get::<i32>("id_user") {
            // id_user présent → on tente le get
            Ok(Some(uid)) => {
                if let Some(user) = User::get(uid)? {
                    Ok(user)
                } else {
                    Err(format!(
                        "The id_user exist in the session but not user is related id:{uid}"
                    ))?
                }
            }

            // pas d'id_user en session → None
            Ok(None) => Err("Infallible call failed fetach the user from session")?,

            // erreur d'accès à la session
            Err(e) => {
                log!("error when accessing session id_user: {e}");
                Err(Box::new(e))
            }
        }
    }
}
impl User {
    pub fn is_admin(&self) -> bool {
        self.admin != 0
    }
    pub fn cart_id(&self) -> DynResult<i32> {
        let mut conn = DB_POOL.get()?;
        let result: Option<i32> = orders::table
            .filter(orders::id_user.eq(self.id_user))
            .filter(orders::date_order.is_null())
            .filter(orders::date_retrieve.is_null())
            .select(orders::id_order)
            .first(&mut conn)
            .optional()?;
        if let Some(id) = result {
            return Ok(id);
        }
        Ok(Order::create_order_for_user(self.id_user)?)
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User:( mail:{}, phone:{}, admin:{} )",
            &self.mail,
            &self.phone_number,
            self.admin != 0
        )
    }
}
