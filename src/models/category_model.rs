use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use crate::{
    schema::{
        categories,
        category_product::{self, dsl::*},
    },
    statics::DB_POOL,
    utilities::{DynResult, handle_optional_query_result},
};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id_category: i32,
    pub name: String,
    pub description: String,
    pub super_category: Option<i32>,
}
#[derive(Serialize, Debug)]
pub struct CategoryGroup {
    pub parent: Category,
    pub children: Vec<Category>,
}
use crate::schema::categories::dsl::*;
use diesel::prelude::*;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=categories)]
pub struct NewCategory {
    pub name: String,
    pub description: String,
    pub super_category: Option<i32>,
}
impl NewCategory {
    pub fn insert(self) -> DynResult<()> {
        let mut conn = DB_POOL.get()?;
        diesel::insert_into(categories)
            .values(&self)
            .execute(&mut conn)?;
        Ok(())
    }
}

#[derive(Deserialize, AsChangeset, Serialize, Insertable)]
#[diesel(table_name=categories)]
pub struct CategoryUpdate {
    pub name: String,
    pub description: String,
}

impl Category {
    pub fn all() -> DynResult<Vec<Category>> {
        let mut conn = DB_POOL.get().map_err(|_| diesel::result::Error::NotFound)?;

        let orphan_id = ORPHAN.id_category;
        let temp = categories
            .filter(categories::id_category.ne(orphan_id))
            .load(&mut conn)?;
        Ok(temp)
    }
    pub fn all_with_orphan() -> DynResult<Vec<Category>> {
        let mut conn = DB_POOL.get()?;
        let temp = categories.load(&mut conn)?;
        Ok(temp)
    }
    pub fn get(id: i32) -> DynResult<Option<Self>> {
        let mut conn = DB_POOL.get()?;

        handle_optional_query_result(
            categories
                .filter(categories::id_category.eq(id))
                .first(&mut conn),
            "Error when trying to get a Category",
        )
    }
    pub fn update(datas: CategoryUpdate, id: i32) -> DynResult<bool> {
        let mut conn = DB_POOL.get()?;
        let res = handle_optional_query_result(
            diesel::update(categories.filter(categories::id_category.eq(id)))
                .set(&datas)
                .execute(&mut conn),
            "Error when trying to update the category",
        )?;
        Ok(res.is_some())
    }

    pub fn load_grouped_categories() -> DynResult<Vec<CategoryGroup>> {
        let all: Vec<Category> = Category::all()?;
        let (parents, children): (Vec<_>, Vec<_>) =
            all.into_iter().partition(|c| c.super_category.is_none());

        let mut output = Vec::with_capacity(parents.len());

        for parent in parents {
            let kids: Vec<Category> = children
                .iter()
                .filter(|&c| c.super_category == Some(parent.id_category))
                .cloned()
                .collect();

            output.push(CategoryGroup {
                parent,
                children: kids,
            });
        }

        Ok(output)
    }
    pub fn all_super_category() -> DynResult<Vec<Category>> {
        let mut conn = DB_POOL.get()?;
        Ok(categories
            .filter(
                categories::super_category
                    .is_null()
                    .and(categories::name.not_like(&ORPHAN.name)),
            )
            .load(&mut conn)?)
    }

    ///Err if there is a real problem Ok(false) if the object do not exist and Ok(true) if the object is delete
    pub fn destroy(id: i32) -> DynResult<bool> {
        let mut conn = DB_POOL.get()?;
        let res =
            diesel::delete(categories.filter(categories::id_category.eq(id))).execute(&mut conn)?;
        Ok(res != 0)
    }
    pub fn orphans() -> DynResult<Vec<Self>> {
        let mut conn = DB_POOL.get()?;
        let orphan_id = ORPHAN.id_category;
        Ok(categories
            .filter(super_category.eq(orphan_id))
            .load(&mut conn)?)
    }

    pub fn all_normal() -> DynResult<Vec<Self>> {
        let mut conn = DB_POOL.get()?;
        Ok(categories
            .filter(super_category.is_not_null())
            .load(&mut conn)?)
    }
    pub fn related_to_product(product_id: i32) -> DynResult<HashSet<i32>> {
        let mut conn = DB_POOL.get()?;
        let ids: Vec<i32> = category_product
            .filter(category_product::id_product.eq(product_id))
            .select(category_product::id_category)
            .load(&mut conn)?;
        Ok(HashSet::from_iter(ids.into_iter()))
    }
}
pub static ORPHAN: LazyLock<Category> = LazyLock::new(|| {
    categories
        .filter(categories::name.eq("__orphan__"))
        .first(&mut DB_POOL.get().unwrap())
        .unwrap()
});
