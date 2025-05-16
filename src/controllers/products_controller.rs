use std::ffi::OsStr;

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Serialize;
use tera::Context;
use uuid::uuid;

use crate::{
    TERA,
    models::{
        category_model::{Category, CategoryGroup},
        product_model::Product,
    },
    routes::{ROUTE_CONTEXT, ROUTE_EDIT_PRODUCT, ROUTE_PRODUCTS},
    utilities::{self, Renderable, Responseable, new_internal_error, render_to_response},
};
#[derive(Serialize, Default)]
pub struct Products {
    products: Vec<Product>,
    is_admin: bool,
    category_groups: Vec<CategoryGroup>,
}

impl Renderable for Products {
    fn render(&self) -> Result<String, tera::Error> {
        let mut context = ROUTE_CONTEXT.clone();
        context.insert("products", &self.products);
        context.insert("is_admin", &self.is_admin);
        context.insert("category_groups", &self.category_groups);
        TERA.render(ROUTE_PRODUCTS.file_path, &context)
    }
}
impl Responseable for Products {}

pub async fn products_get(session: Session) -> impl Responder {
    let all = Product::all();
    let Some(products) = all else {
        return new_internal_error();
    };
    let Ok(category_groups) = Category::load_grouped_categories() else {
        println!("the problem is here !");
        return new_internal_error();
    };

    let products_view = Products {
        products: products,
        is_admin: utilities::is_admin(&session),
        category_groups: category_groups,
    };

    products_view.into_response()
}

pub async fn product_id_get(path: web::Path<i32>) -> impl Responder {
    println!("product_id_get called");
    let mut context = Context::new();
    context.extend(ROUTE_CONTEXT.clone());
    let product = if let Some(p) = Product::get(*path) {
        p
    } else {
        return HttpResponse::NotFound().body("No product exist with the given id");
    };
    context.insert("name", &product.name);
    context.insert("id_product", &product.id_product);
    context.insert("description", &product.description);
    context.insert("price", &product.price);
    context.insert("image_url", &product.image_url);

    render_to_response(TERA.render(ROUTE_EDIT_PRODUCT.file_path, &context))
}

pub async fn product_id_delete(path: web::Path<i32>) -> impl Responder {
    if Product::delete(path.into_inner()) {
        HttpResponse::Ok().body(
            r###"
            <html>
                <body>
                    <h1>Le produit a été suprimer</h1>
                </body>
            </html>
            "###,
        )
    } else {
        HttpResponse::NoContent().body(
            r###"
            <html>
                <body>
                    <h1>Le produit n'as pas été trouver</h1>
                </body>
            </html>
            "###,
        )
    }
}

#[derive(Debug, MultipartForm)]
pub struct ProductEditForm {
    pub id_product: Text<i32>,
    pub name: Text<String>,
    pub description: Text<String>,
    pub price: Text<f64>,

    #[multipart(rename = "image_file", limit = "10MB")]
    pub image: Option<TempFile>,
}
async fn product_id_patch(MultipartForm(form): MultipartForm<ProductEditForm>) -> impl Responder {
    let id = form.id_product.0;
    let name = form.name.0;
    let description = form.description.0;
    let price = form.price.0;
    let uuid_part = uuid::Uuid::new_v4();

    let image_path = if let Some(temp_image) = form.image {
        let filename = format!(
            "uploads/product_{uuid_part}.{}",
            get_extension_from_filename(&temp_image.file_name.unwrap_or_default())
                .unwrap_or_default()
        );
        temp_image.file.persist(&filename).unwrap();
        Some(filename)
    } else {
        None
    };

    // 👉 Ici, tu peux appeler ton update Diesel avec id, name, description, price, image_path

    HttpResponse::Ok().body(format!(
        "Produit modifié : {} - {} - {} € - image: {:?}",
        name, description, price, image_path
    ))
}
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}
