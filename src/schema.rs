diesel::table! {
    categories (id_categories) {
        id_categories -> Integer,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    category_product (id_products, id_categories) {
        id_products -> Integer,
        id_categories -> Integer,
    }
}

diesel::table! {
    products (id_products) {
        id_products -> Integer,
        description -> Text,
        name -> Text,
        price -> Double,
        image_url -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(categories, category_product, products,);
