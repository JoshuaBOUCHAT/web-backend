use crate::schema::category_product::{self, dsl::*};
use crate::{statics::DB_POOL, utilities::DynResult};
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::prelude::{Insertable, Queryable};
use diesel::query_dsl::methods::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = category_product)]
pub struct CategoryProduct {
    pub id_product: i32,
    pub id_category: i32,
}
impl CategoryProduct {
    pub fn all() -> DynResult<Vec<Self>> {
        let mut conn = DB_POOL.get()?;
        Ok(category_product.load(&mut conn)?)
    }
    pub fn bulk_delete(product_id: i32, cats: &[i32]) -> DynResult<usize> {
        if cats.is_empty() {
            return Ok(0); // rien à faire
        }

        let mut conn = DB_POOL.get()?;
        let n = diesel::delete(
            category_product
                .filter(category_product::id_product.eq(product_id))
                .filter(category_product::id_category.eq_any(cats)),
        )
        .execute(&mut conn)?;
        Ok(n) // nombre de lignes supprimées
    }

    // ────────────────────────────────────────────────────────────────
    /// Insère en bloc (ignore les doublons si déjà présents)
    pub fn bulk_insert(product_id: i32, cats: &[i32]) -> DynResult<usize> {
        if cats.is_empty() {
            return Ok(0); // rien à insérer
        }

        // Prépare les lignes à insérer
        let new_rows: Vec<CategoryProduct> = cats
            .iter()
            .map(|&cid| CategoryProduct {
                id_product: product_id,
                id_category: cid,
            })
            .collect();

        let mut conn = DB_POOL.get()?;
        let n = diesel::insert_into(category_product)
            .values(&new_rows)
            // ← ignore si la paire existe déjà
            .execute(&mut conn)?;
        Ok(n) // nombre de lignes réellement insérées
    }
}
