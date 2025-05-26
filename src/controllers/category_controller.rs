use actix_web::{
    HttpResponse,
    web::{self, Form},
};
use tera::Context;

use crate::{
    log,
    models::{
        self,
        category_model::{Category, CategoryUpdate, NewCategory, ORPHAN},
    },
    routes::{ROUTE_CATEGORY_EDIT, ROUTE_CATEGORY_NEW, ROUTE_CATEGORY_SELECT, ROUTE_CONTEXT},
    statics::TERA,
    utilities::{DynResult, render_to_response},
};

pub async fn new_post(Form(form): Form<CategoryForm>) -> DynResult<HttpResponse> {
    form.into_insertable()?.insert()?;

    Ok(HttpResponse::Ok().body("La création de la nouvelle catégorie est un succés"))
}
#[derive(serde::Deserialize)]
pub struct CategoryForm {
    name: String,
    description: String,
    super_category: String,
}
impl CategoryForm {
    fn into_insertable(self) -> DynResult<NewCategory> {
        let id = match self.super_category.as_str() {
            "__orphan__" => Some(ORPHAN.id_category),
            "super_category" => None,
            s => Some(s.parse()?),
        };
        Ok(NewCategory {
            description: self.description,
            name: self.name,
            super_category: id,
        })
    }
}

pub async fn new_get() -> DynResult<HttpResponse> {
    let mut context = ROUTE_CONTEXT.clone();
    let super_categories = Category::all_super_category()?;
    eprintln!("{:?}", &super_categories);
    context.insert("super_categories", &super_categories);
    Ok(render_to_response(
        TERA.render(ROUTE_CATEGORY_NEW.file_path, &context),
    ))
}

pub async fn edit_post(
    Form(form): Form<CategoryUpdate>,
    path: web::Path<i32>,
) -> DynResult<HttpResponse> {
    let id = *path;
    let success = Category::update(form, id)?;
    let response = if success {
        HttpResponse::Ok().body("The category is updated successfully")
    } else {
        log!("Attempt to modify somthing that didn't existe");
        HttpResponse::NotFound().body("The category you tried to modify do not existe")
    };
    Ok(response)
}

pub async fn edit_get(path: web::Path<i32>) -> DynResult<HttpResponse> {
    let mut context = ROUTE_CONTEXT.clone();
    let category_id = *path;
    context.insert("current_super_category_id", &category_id);
    let Some(category) = Category::get(category_id)? else {
        log!("The category is none but it should be Some. In edit get");
        return Err("Category is empty wher it should not in edit_get")?;
    };
    context.insert("category", &category);
    let id_orphan = models::category_model::ORPHAN.id_category;

    let category_type = match category.super_category {
        None => "Super Catégorie",
        Some(t) if t == id_orphan => "normal",
        Some(t) => {
            if let Some(cat) = Category::get(t)? {
                &cat.name.to_string()
            } else {
                "undefined"
            }
        }
    };
    context.insert("category_type", category_type);

    Ok(render_to_response(
        TERA.render(ROUTE_CATEGORY_EDIT.file_path, &context),
    ))
}

pub async fn select_get() -> DynResult<HttpResponse> {
    let mut context = Context::new();
    context.insert("categories", &Category::all()?);
    Ok(render_to_response(
        TERA.render(ROUTE_CATEGORY_SELECT.file_path, &context),
    ))
}

pub async fn destroy(path: web::Path<i32>) -> DynResult<HttpResponse> {
    let id = *path;
    let response = if Category::destroy(id)? {
        HttpResponse::Ok().body("The category has been destoyed succesfully")
    } else {
        HttpResponse::BadRequest().body("The category that should be destroyed do no existe")
    };
    Ok(response)
}
