use crate::{schema::categories, statics::DB_POOL};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id_category: i32,
    pub name: String,
    pub description: String,
    pub super_category: Option<i32>,
}
#[derive(Serialize)]
pub struct CategoryGroup {
    pub parent: Category,
    pub children: Vec<Category>,
}
use crate::schema::categories::dsl::*;
use diesel::prelude::*;

impl Category {
    pub fn all() -> diesel::QueryResult<Vec<Category>> {
        let mut conn = DB_POOL.get().map_err(|_| diesel::result::Error::NotFound)?;
        println!("tentative du desp");

        // 1. Récupérer toutes les catégories
        let temp = categories.load(&mut conn);
        if let Err(err) = &temp {
            println!("err:{err}");
        }
        temp
    }
    pub fn load_grouped_categories() -> diesel::QueryResult<Vec<CategoryGroup>> {
        println!("sheeeeehs");
        let all: Vec<Category> = Category::all()?;
        println!("get all worked");
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
}
