use actix_session::Session;
use actix_web::{
    HttpResponse,
    web::{self, Path},
};
use tera::Context;

use crate::{
    models::{
        order_model::{Order, OrderState},
        order_product_model::OrderProduct,
        product_model::Product,
        user_model::User,
    },
    routes::{ROUTE_DASHBOARD, ROUTE_ORDER},
    statics::TERA,
    utilities::{DynResult, render_to_response, send_mail},
};

pub async fn update(path: web::Path<(i32, i32)>, session: Session) -> DynResult<HttpResponse> {
    let (product_id, qty) = *(path);
    let user = User::from_session_infallible(&session)?;
    let order_id = user.cart_id()?;
    println!("cart_id:{order_id}");
    OrderProduct::update(order_id, product_id, qty)?;

    let message = if qty == 1 {
        "Le  produit à bien été ajouté"
    } else {
        "Les produits on bien été ajouté"
    };
    Ok(HttpResponse::Ok().body(message))
}
pub async fn edit(path: Path<i32>, session: Session) -> DynResult<HttpResponse> {
    let user = User::from_session_infallible(&session)?;
    let id_product = *path;

    let Some(product) = Product::get(id_product)? else {
        return Err("The product do not exist !")?;
    };

    let obj = OrderProduct::qty_from_cart_and_product(user.cart_id()?, product.id_product)?;
    let (already_in_cart, qty) = if let Some(qty) = obj {
        (true, qty)
    } else {
        (false, 1)
    };
    let mut context = Context::new();
    context.insert("qty", &qty);
    context.insert("already_in_cart", &already_in_cart);
    context.insert("id_product", &product.id_product);
    context.insert("name", &product.name);
    context.insert("description", &product.description);

    Ok(render_to_response(
        TERA.render(ROUTE_ORDER.file_path, &context),
    ))
}

pub async fn destroy(path: web::Path<i32>, session: Session) -> DynResult<HttpResponse> {
    let product_id = *path;
    let user = User::from_session_infallible(&session)?;
    let cart_id = user.cart_id()?;
    println!("cart_id: {cart_id}");
    let is_deleted = OrderProduct::delete(cart_id, product_id)?;

    let resp = if is_deleted {
        HttpResponse::Ok().body("Le produit à été suprimer du panier")
    } else {
        HttpResponse::BadRequest().body("Une erreur est survenu lors de la supression")
    };
    Ok(resp)
}

const STATES_CHOICE: [&str; 3] = ["confirm", "ready", "complete"];

pub async fn update_state(path: web::Path<(i32, String)>) -> DynResult<HttpResponse> {
    let order_id = path.0;
    let state = path.1.as_str();
    let order = Order::get(order_id)?.unwrap();
    let user = User::get(order.id_user)?.unwrap();

    if !STATES_CHOICE.contains(&state) {
        return Err("Invalid state")?;
    }
    let state = match state {
        "confirm" => OrderState::Confirmed,
        "ready" => OrderState::Ready,
        "complete" => OrderState::Purnchased,
        _ => return Err("Invalid state")?,
    };
    Order::update_state(order_id, state)?;

    let (subject, body) = match state {
        OrderState::Confirmed => (
            SUBJECT_CONFIRMED,
            get_order_confirmed_mail_body(order_id, &order.date_retrieve.unwrap()),
        ),
        OrderState::Ready => (SUBJECT_READY, get_order_ready_mail_body(order_id)),
        OrderState::Purnchased => (SUBJECT_COMPLETED, get_order_completed_mail_body(order_id)),
        _ => return Err("Invalid state")?,
    };
    send_mail(&user.mail, subject, &body)?;

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", ROUTE_DASHBOARD.web_path))
        .finish())
}

#[derive(Debug, serde::Deserialize)]
pub struct RefuseForm {
    reason: String,
}

pub async fn refuse(
    path: web::Path<i32>,
    json_form: web::Json<RefuseForm>,
) -> DynResult<HttpResponse> {
    let order_id = *path;
    let order = Order::get(order_id)?.unwrap();
    let user = User::get(order.id_user)?.unwrap();
    let state = OrderState::Refused;
    Order::update_state(order_id, state)?;
    let subject = SUBJECT_REFUSED;
    let body = get_order_refused_mail_body(order_id, &json_form.reason);
    send_mail(&user.mail, subject, &body)?;
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", ROUTE_DASHBOARD.web_path))
        .finish())
}

const SUBJECT_CONFIRMED: &str = "Votre commande a été confirmée – Boulangerie La Traditionnelle";

fn get_order_confirmed_mail_body(order_id: i32, date_retrieve: &str) -> String {
    format!(
        r###"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
    </head>
    <body style="font-family: Arial, sans-serif; color: #333;">
        <h2 style="color: #6e4b3a;">Commande confirmée 🧾</h2>
        <p>Bonjour,</p>
        <p>Votre commande n°<strong>{}</strong> a été <strong>confirmée</strong> avec succès.</p>
        <p>Elle sera disponible à partir du <strong>{}</strong> à la Boulangerie La Traditionnelle.</p>
        <p>Merci pour votre confiance !</p>
        <p>À très bientôt,<br>L’équipe de la Boulangerie La Traditionnelle 🥐</p>
    </body>
    </html>"###,
        order_id, date_retrieve
    )
}
const SUBJECT_READY: &str = "Votre commande est prête – Boulangerie La Traditionnelle";

fn get_order_ready_mail_body(order_id: i32) -> String {
    format!(
        r###"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
    </head>
    <body style="font-family: Arial, sans-serif; color: #333;">
        <h2 style="color: #6e4b3a;">Commande prête 🎉</h2>
        <p>Bonjour,</p>
        <p>Votre commande n°<strong>{}</strong> est <strong>prête</strong> et vous attend à la boulangerie !</p>
        <p>Nous vous remercions pour votre commande.</p>
        <p>À tout de suite !<br>L’équipe de la Boulangerie La Traditionnelle 🥖</p>
    </body>
    </html>"###,
        order_id
    )
}

const SUBJECT_COMPLETED: &str = "Merci pour votre commande – Boulangerie La Traditionnelle";

fn get_order_completed_mail_body(order_id: i32) -> String {
    format!(
        r###"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
    </head>
    <body style="font-family: Arial, sans-serif; color: #333;">
        <h2 style="color: #6e4b3a;">Merci pour votre commande 🙌</h2>
        <p>Bonjour,</p>
        <p>Votre commande n°<strong>{}</strong> a bien été <strong>retirée</strong>.</p>
        <p>Nous espérons que tout était à votre goût !</p>
        <p>À très bientôt pour de nouvelles gourmandises 🍞</p>
        <p>L’équipe de la Boulangerie La Traditionnelle</p>
    </body>
    </html>"###,
        order_id
    )
}

const SUBJECT_REFUSED: &str = "Votre commande a été refusée – Boulangerie La Traditionnelle";

fn get_order_refused_mail_body(order_id: i32, reason: &str) -> String {
    format!(
        r###"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
    </head>
    <body style="font-family: Arial, sans-serif; color: #333;">
        <h2 style="color: #a94442;">Commande refusée ❌</h2>
        <p>Bonjour,</p>
        <p>Nous sommes désolés, mais votre commande n°<strong>{}</strong> a été <strong>refusée</strong>.</p>
        <p>Motif : <em>{}</em></p>
        <p>Si vous avez des questions, n’hésitez pas à nous contacter.</p>
        <p>Merci de votre compréhension,<br>L’équipe de la Boulangerie La Traditionnelle</p>
    </body>
    </html>"###,
        order_id, reason
    )
}
