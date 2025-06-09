use std::{collections::HashSet, ffi::OsStr};

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
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
        user_model::MaybeUser,
    },
    routes::{ROUTE_CONTEXT, ROUTE_EDIT_PRODUCT, ROUTE_PRODUCTS},
    statics::TERA,
    utilities::{DynResult, Renderable, Responseable, render_to_response},
};
#[derive(Serialize, Default)]
pub struct Products {
    products: Vec<ProductWithCategories>,
    is_admin: bool,
    category_groups: Vec<CategoryGroup>,
    orphans: Vec<Category>,
    is_connected: bool,
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

pub async fn products_get(maybe_user: MaybeUser) -> DynResult<HttpResponse> {
    let products = load_products_with_categories()?;
    let category_groups = Category::load_grouped_categories()?;
    let orphans = Category::orphans()?;

    let (is_admin, is_connected) = maybe_user
        .0
        .map_or((false, false), |user| (user.is_admin(), true));

    let products_view = Products {
        products,
        is_admin,
        category_groups,
        orphans,
        is_connected,
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
    let image_path = match form.image {
        // we have an actual file (non-zero size and a real filename)
        Some(tmp) if tmp.size > 0 && tmp.file_name.as_deref().is_some_and(|n| !n.is_empty()) => {
            let ext =
                get_extension_from_filename(tmp.file_name.as_deref().unwrap()).unwrap_or("bin");
            let uuid_part = uuid::Uuid::new_v4();

            let filename = format!("public/uploads/product_{uuid_part}.{ext}");
            tmp.file.persist(&filename).unwrap();

            Some(filename)
        }
        _ => None, // no file chosen → keep current image
    };

    let product_id = form.id_product.0;

    let patch_product = ProductPatchBuilder::default()
        .description(Some(form.description.0))
        .image_url(image_path)
        .name(Some(form.name.0))
        .price(Some(form.price.0))
        .build()
        .unwrap();
    let _ = Product::patch(product_id, patch_product)?;

    let relateds = Category::related_to_product(product_id)?;
    // Calcul des catégories à supprimer (présentes en BDD mais plus sélectionnées)

    let category_ids: HashSet<i32> = form.categories.into_iter().map(|c| c.0).collect();

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
) -> DynResult<HttpResponse> {
    let temp_file = form.image;
    let temp_file_name = match temp_file.file_name.as_deref() {
        Some(name) if !name.is_empty() => name,
        _ => return Ok(HttpResponse::BadRequest().body("You need to provide an image")),
    };

    let ext = get_extension_from_filename(temp_file_name).ok_or("File extension non parsable")?;
    let uuid_part = uuid::Uuid::new_v4();
    let image_url = format!("public/uploads/product_{uuid_part}.{ext}");

    temp_file.file.persist(&image_url)?;

    let new_product = NewProduct {
        description: form.description.0,
        name: form.name.0,
        image_url,
        price: form.price.0,
    };
    let _ = Product::create(new_product)?;

    Ok(HttpResponse::Ok().body("Le produit à bien été créer !"))
}

#[derive(Deserialize)]
pub struct VisibilityPayload {
    pub visible: i32,
}

pub async fn product_put_visibility(
    path: web::Path<i32>,
    payload: web::Json<VisibilityPayload>,
) -> DynResult<HttpResponse> {
    let visibility_value: i32 = payload.visible;
    if visibility_value != 0 && visibility_value != 1 {
        return Ok(HttpResponse::BadRequest().body(format!(
            "The provided visibility is wrong: {}",
            visibility_value
        )));
    }

    let response = if Product::update_visibility(*path, visibility_value)? {
        HttpResponse::Ok().body("The visibility of the product has change")
    } else {
        HttpResponse::BadRequest().body("The visibilty did not changed an error occure")
    };
    Ok(response)
}
