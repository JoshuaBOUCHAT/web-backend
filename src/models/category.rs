use crate::schema::categories;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id_category: i32,
    pub name: String,
    pub description: String,
}
