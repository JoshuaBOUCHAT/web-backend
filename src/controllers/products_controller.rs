use std::{collections::HashSet, ffi::OsStr};

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    log,
    models::{
        category_model::{Category, CategoryGroup},
        category_product_model::CategoryProduct,
        complex_request::{ProductWithCategories, load_products_with_categories},
        product_model::{NewProduct, Product, ProductPatchBuilder},
        user_model::User,
    },
    routes::{ROUTE_CONTEXT, ROUTE_EDIT_PRODUCT, ROUTE_PRODUCTS},
    statics::TERA,
    try_or_return,
    utilities::{DynResult, ExtractHttp, Renderable, Responseable, render_to_response},
};
#[derive(Serialize, Default)]
pub struct Products {
    products: Vec<ProductWithCategories>,
    is_admin: bool,
    category_groups: Vec<CategoryGroup>,
    orphans: Vec<Category>,is_connected:bool,

}

impl Renderable for Products {
    fn render(&self) -> Result<String, tera::Error> {
        let mut context = ROUTE_CONTEXT.clone();
        context.insert("products", &self.products);
        context.insert("is_admin", &self.is_admin);
        context.insert("category_groups", &self.category_groups);
        context.insert("orphans", &self.orphans);
        context.insert("is_connected", &self.is_connected);
        TERA.render(ROUTE_PRODUCTS.file_path, &context)
    }
}
impl Responseable for Products {}

pub async fn products_get(session: Session) -> DynResult<HttpResponse> {
    let products = load_products_with_categories()?;
    let category_groups = Category::load_grouped_categories()?;
    let maybe_user = User::from_session(&session)?;
    let orphans = Category::orphans()?;
    let(is_admin,is_connected)=if let Some(user)=maybe_user{
        (user.is_admin(),true)
    }else{
        (false,false)
    };

    let products_view = Products {
        products: products,
        is_admin: is_admin,
        category_groups: category_groups,
        orphans: orphans,
        is_connected: is_connected,
    };

    Ok(products_view.into_response())
}

pub async fn product_id_get(path: web::Path<i32>) -> DynResult<HttpResponse> {
    let mut context = Context::new();
    context.extend(ROUTE_CONTEXT.clone());
    let maybe_product = Product::get(*path)?;
    let Some(product) = maybe_product else {
        log!("Trying to access a non-existent product at id :{}", *path);
        return Ok(HttpResponse::NotFound().body("Product not found"));
    };
    let categories = Category::all_normal()?;
    let relateds = Category::related_to_product(product.id_product)?;

    context.insert("name", &product.name);
    context.insert("id_product", &product.id_product);
    context.insert("description", &product.description);
    context.insert("price", &product.price);
    context.insert("image_url", &product.image_url);
    context.insert("categories", &categories);
    context.insert("relateds", &relateds);

    Ok(render_to_response(
        TERA.render(ROUTE_EDIT_PRODUCT.file_path, &context),
    ))
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
    #[multipart(rename = "categories[]")]
    pub categories: Vec<Text<i32>>,

    #[multipart(rename = "image_file", limit = "10MB")]
    pub image: Option<TempFile>,
}
pub async fn product_id_patch(
    MultipartForm(form): MultipartForm<ProductEditForm>,
) -> DynResult<HttpResponse> {
    println!("pass at product_id_patch");
    let product_id = form.id_product.0;
    let name = form.name.0;
    let description = form.description.0;
    let uuid_part = uuid::Uuid::new_v4();
    let category_ids: HashSet<i32> = form.categories.into_iter().map(|c| c.0).collect();

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
    let _ = Product::patch(product_id, patch_product)?;

    let relateds = Category::related_to_product(product_id)?;
    // Calcul des catégories à supprimer (présentes en BDD mais plus sélectionnées)
    let to_delete: Vec<i32> = relateds
        .iter()
        .filter(|&&p| !category_ids.contains(&p))
        .copied()
        .collect();
    // Calcul des catégories à ajouter (sélectionnées mais pas encore en BDD)
    let to_create: Vec<i32> = category_ids
        .into_iter()
        .filter(|&i| !relateds.contains(&i))
        .collect();

    let deleted = CategoryProduct::bulk_delete(product_id, &to_delete)?;
    let created = CategoryProduct::bulk_insert(product_id, &to_create)?;
    if created != to_create.len() || deleted != to_delete.len() {
        log!(
            "Error wrong number of insert/delete inside the edit product handle form created: {}|{}    deleted: {}|{}",
            created,
            to_create.len(),
            deleted,
            to_delete.len()
        );
    }

    Ok(HttpResponse::Ok().finish())
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
