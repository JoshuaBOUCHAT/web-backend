use crate::{
    models::user_model::User,
    statics::{APP_URL, DB_POOL},
    utilities::{DynResult, handle_optional_query_result, now_str},
};
use chrono::{Duration, Utc};
use diesel::{Selectable, prelude::*};
use uuid::Uuid;

#[derive(Selectable, Queryable, Debug)]
#[diesel(table_name =crate::schema::email_verifications)]
pub struct EmailVerification {
    pub id_email_verification: i32,
    pub id_user: i32,
    pub expiration: String,
    pub token: String,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name =crate::schema::email_verifications)]
pub struct InsertVerification {
    pub id_user: i32,
    pub expiration: String,
    pub token: String,
}

impl EmailVerification {
    pub fn verify(token_test: &str) -> DynResult<VerificationState> {
        use crate::schema::email_verifications::dsl::*;
        let mut conn = DB_POOL.get()?;
        let querry = email_verifications
            .filter(token.eq(token_test))
            .first(&mut conn);
        let maybe_verif: Option<EmailVerification> =
            handle_optional_query_result(querry, "Error happen when querring email verification")?;

        let Some(verif) = maybe_verif else {
            return Ok(VerificationState::WrongToken);
        };
        let now = now_str();
        if now > verif.expiration {
            return Ok(VerificationState::Expired(verif.id_user));
        };
        //Place verified == 1 dans pour le user
        User::set_verified(verif.id_user)?;

        Ok(VerificationState::Verified(verif.id_user))
    }

    // Une date d'expiration de 2 jours

    pub fn create(user_id: i32) -> DynResult<String> {
        use crate::schema::email_verifications::dsl::*;
        let mut conn = DB_POOL.get()?;

        // Générer une date d'expiration à +2 jours
        let expiration_date = (Utc::now() + Duration::days(2))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // Générer un token unique
        let generated_token = Uuid::new_v4().to_string();

        // Créer la structure à insérer
        let new_entry = InsertVerification {
            id_user: user_id,
            expiration: expiration_date,
            token: generated_token.clone(),
        };

        // Insérer dans la base de données
        diesel::insert_into(email_verifications)
            .values(&new_entry)
            .execute(&mut conn)?;

        // Le liens

        let link = format!("{}/auth/verify?token={}", &*APP_URL, generated_token);
        Ok(link)
    }
    pub fn ensure_valide_mail(user_id: i32) -> DynResult<Option<String>> {
        use crate::schema::email_verifications::{self, dsl::*};
        let mut conn = DB_POOL.get()?;
        let now = now_str();
        let maybe_verif: Option<EmailVerification> = handle_optional_query_result(
            email_verifications
                .filter(email_verifications::id_user.eq(user_id))
                //.filter(expiration.gt(now))
                .first(&mut conn),
            "err message ",
        )?;

        let res = if let Some(verif) = maybe_verif {
            println!("verfi found: {:?}", &verif);
            None
        } else {
            println!("id:{user_id}");
            let new_link = Self::create(user_id)?;
            println!("create new link");
            Some(new_link)
        };
        Ok(res)
    }
}
pub enum VerificationState {
    Verified(i32),
    WrongToken,
    Expired(i32),
}
