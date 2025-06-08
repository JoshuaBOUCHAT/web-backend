use actix_session::Session;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use tera::Context;

use crate::{
    models::{
        user_model::User,
        verify_model::{EmailVerification, VerificationState},
    },
    routes::{ROUTE_AUTH, ROUTE_CONTEXT, ROUTE_PRODUCTS, ROUTE_VERIFY},
    statics::TERA,
    utilities::{DynResult, render_to_response, send_mail},
};

#[derive(Deserialize)]
pub struct Token {
    token: Option<String>,
}

pub async fn index(querry: web::Query<Token>, session: Session) -> DynResult<HttpResponse> {
    let maybe_token = &querry.token;
    eprintln!("handle verify");
    if let Some(token) = maybe_token {
        eprintln!("handle token");
        let response = match EmailVerification::verify(token)? {
            //redirect to auth if the user is connected will redirect to products
            VerificationState::Verified(_) => HttpResponse::Found()
                .append_header(("Location", ROUTE_AUTH.web_path))
                .finish(),
            VerificationState::Expired(user_id) => {
                let link = EmailVerification::create(user_id)?;
                let user = User::get(user_id)?.unwrap();
                let mut context = Context::new();
                context.insert(
                "message",
                "Le lien est expiré veuillez vérfié votre boite mail car un nouveau a été envoyé",
                );
                context.extend(ROUTE_CONTEXT.clone());
                send_verification_mail(&link, &user.mail)?;
                render_to_response(TERA.render(ROUTE_VERIFY.file_path, &context))
            }
            VerificationState::WrongToken => {
                let mut context = Context::new();
                context.insert(
                    "message",
                    "Le lien est invalide assurer vous de bien l'avoir ouvert",
                );
                context.extend(ROUTE_CONTEXT.clone());
                render_to_response(TERA.render(ROUTE_VERIFY.file_path, &context))
            }
        };
        return Ok(response);
    };
    println!("No token found");
    let Some(user) = User::from_session(&session)? else {
        println!("redirect to auth");
        // The user isn't connected
        return Ok(HttpResponse::Found()
            .append_header(("Location", ROUTE_AUTH.web_path))
            .finish());
    };
    if user.verified != 0 {
        //user already verified redirect to prodcuts page
        println!("redirect to products");
        return Ok(HttpResponse::Found()
            .append_header(("Location", ROUTE_PRODUCTS.web_path))
            .finish());
    }
    println!("verifying if a valide link existe");
    let Some(link_url) = EmailVerification::ensure_valide_mail(user.id_user)? else {
        //still work
        println!("link still work");
        let mut context = Context::new();
        context.insert(
            "message",
            "Un lien vous à été envoyé par mail pour vérifier votre compte (n'oublié pas de verifier les spame si vous ne le trouvez pas)",
        );
        context.extend(ROUTE_CONTEXT.clone());
        return Ok(render_to_response(
            TERA.render(ROUTE_VERIFY.file_path, &context),
        ));
    };
    //case where the link expired
    println!("link not working anymore sending new one");

    send_verification_mail(&link_url, &user.mail)?;
    println!("sended");
    let mut context = Context::new();
    context.insert(
        "message",
        "Un lien vous été envoyer plus aucun lien valide existait",
    );
    context.extend(ROUTE_CONTEXT.clone());
    Ok(render_to_response(
        TERA.render(ROUTE_VERIFY.file_path, &context),
    ))
}
fn send_verification_mail(link: &str, destination: &str) -> DynResult<()> {
    let body = get_mail_body(link);

    send_mail(destination, SUBJECT, body)?;
    Ok(())
}

const SUBJECT: &str = r##"Confirmez votre adresse e-mail – Boulangerie La Traditionnelle"##;

fn get_mail_body(link: &str) -> String {
    format!(
        r###"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
    </head>
    <body style="font-family: Arial, sans-serif; color: #333;">
        <h2 style="color: #6e4b3a;">Bienvenue à la Boulangerie La Traditionnelle 🥖</h2>
        <p>Bonjour,</p>
        <p>Merci de vous être inscrit ! Pour finaliser votre inscription, veuillez confirmer votre adresse e-mail en cliquant sur le lien ci-dessous :</p>
        <p>
            <a href="{}" style="background-color: #d2a679; color: white; padding: 10px 15px; border-radius: 5px; text-decoration: none;">
                Confirmer mon adresse e-mail
            </a>
        </p>
        <p>Ce lien expirera dans 48 heures.</p>
        <p>Si vous n’avez pas créé de compte, vous pouvez ignorer cet e-mail.</p>
        <p>Merci et à très bientôt !<br>L’équipe de la Boulangerie La Traditionnelle</p>
    </body>
    </html>"###,
        link
    )
}
