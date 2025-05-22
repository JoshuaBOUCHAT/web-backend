use std::ffi::OsStr;

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    log,
    models::{
        category_model::{Category, CategoryGroup},
        complex_request::{ProductWithCategories, load_products_with_categories},
        product_model::{NewProduct, Product, ProductPatchBuilder},
        user_model::User,
    },
    routes::{ROUTE_CONTEXT, ROUTE_EDIT_PRODUCT, ROUTE_PRODUCTS},
    statics::TERA,
    try_or_return,
    utilities::{ExtractHttp, Renderable, Responseable, render_to_response},
};
#[derive(Serialize, Default)]
pub struct Products {
    products: Vec<ProductWithCategories>,
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
    let products = try_or_return!(
        load_products_with_categories().extract_http(),
        log!("Error when loading products in products_get\n")
    );
    let category_groups = try_or_return!(
        Category::load_grouped_categories().extract_http(),
        log!("Error when loading categories in products_get\n")
    );

    let maybe_user = try_or_return!(
        User::from_session(&session).extract_http(),
        log!("Error happen during rendering of products_get while requesting User form session ")
    );
    let is_admin = if let Some(user) = maybe_user {
        user.admin != 0
    } else {
        false
    };

    let products_view = Products {
        products: products,
        is_admin: is_admin,
        category_groups: category_groups,
    };

    products_view.into_response()
}

pub async fn product_id_get(path: web::Path<i32>) -> impl Responder {
    let mut context = Context::new();
    context.extend(ROUTE_CONTEXT.clone());
    let maybe_product = try_or_return!(
        Product::get(*path).extract_http(),
        log!("Error happen while trying to render product_id_get")
    );
    let Some(product) = maybe_product else {
        log!("Trying to access a non-existent product at id :{}", *path);
        return HttpResponse::NotFound().body("Product not found");
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
pub async fn product_id_patch(
    MultipartForm(form): MultipartForm<ProductEditForm>,
) -> impl Responder {
    println!("pass at product_id_patch");
    let id = form.id_product.0;
    let name = form.name.0;
    let description = form.description.0;
    let uuid_part = uuid::Uuid::new_v4();

    let image_path = match form.image {
        // we have an actual file (non-zero size and a real filename)
        Some(tmp) if tmp.size > 0 && tmp.file_name.as_deref().map_or(false, |n| !n.is_empty()) => {
            let ext =
                get_extension_from_filename(tmp.file_name.as_deref().unwrap()).unwrap_or("bin");
            let filename = format!("public/uploads/product_{uuid_part}.{ext}");
            tmp.file.persist(&filename).unwrap();
            Some(filename)
        }
        _ => None, // no file chosen → keep current image
    };
    let patch_product = ProductPatchBuilder::default()
        .description(Some(description))
        .image_url(image_path)
        .name(Some(name))
        .price(Some(form.price.0))
        .build()
        .unwrap();
    let result = Product::patch(id, patch_product);

    let _ = try_or_return!(
        result.extract_http(),
        log!("Error append when patching a the product with id :{id}")
    );
    /*HttpResponse::SeeOther()
    .append_header(("Location", ROUTE_PRODUCTS.web_path))
    .finish()*/
    HttpResponse::Ok().finish()
}
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

#[derive(Debug, MultipartForm)]
pub struct ProductCreateForm {
    pub name: Text<String>,
    pub description: Text<String>,
    pub price: Text<f64>,

    #[multipart(rename = "image_file", limit = "10MB")]
    pub image: TempFile,
}

pub async fn product_post(
    MultipartForm(form): MultipartForm<ProductCreateForm>,
    session: Session,
) -> impl Responder {
    println!("pass at product_post");
    let form_desc = format!("{:?}", &form);
    let name = &form.name.0;
    let description = form.description.0;
    let uuid_part = uuid::Uuid::new_v4();

    let tmp = form.image;

    let image_path = if tmp.size > 0 && tmp.file_name.as_deref().map_or(false, |n| !n.is_empty()) {
        let ext = get_extension_from_filename(tmp.file_name.as_deref().unwrap()).unwrap_or("bin");
        let filename = format!("public/uploads/product_{uuid_part}.{ext}");
        tmp.file.persist(&filename).unwrap();
        filename
    } else {
        let maybe_user = try_or_return!(User::from_session(&session).extract_http());
        if let Some(user) = maybe_user {
            log!(
                "Tried to create a product wihout an image while it's required, the user is: {user}"
            );
        } else {
            log!(
                "Tried to create a product wihout an image while it's required, the user can't be found"
            );
        };
        return HttpResponse::BadRequest().body("You need to provide an image");
    }; // no file uploaded
    let new_product = NewProduct {
        description: description,
        name: name.clone(),
        image_url: image_path,
        price: form.price.0,
    };
    let result = Product::create(new_product);

    let _ = try_or_return!(
        result.extract_http(),
        log!(
            "Error append when insering a new product the formData:{}",
            form_desc
        )
    );
    /*HttpResponse::SeeOther()
    .append_header(("Location", ROUTE_PRODUCTS.web_path))
    .finish()*/
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct VisibilityPayload {
    pub visible: i32,
}

pub async fn product_put_visibility(
    path: web::Path<i32>,
    payload: web::Json<VisibilityPayload>,
) -> impl Responder {
    let visibility_value: i32 = payload.visible;
    if visibility_value != 0 && visibility_value != 1 {
        return HttpResponse::BadRequest().body(format!(
            "The provided visibility is wrong: {}",
            visibility_value
        ));
    }
    let b = try_or_return!(Product::update_visibility(*path, visibility_value).extract_http());

    if b {
        HttpResponse::Ok().body("The visibility of the product has change")
    } else {
        HttpResponse::BadRequest().body("The visibilty did not changed an error occure")
    }
}
