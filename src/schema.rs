use diesel::table;

table! {
    products (id_products) {
        id_products -> Int4,
        description -> Text,
        name -> Text,
        price -> Double,
        image_url -> Text,
    }
}

table! {
    categories (id_categories) {
        id_categories -> Int4,
        name -> Text,
        description -> Text,
    }
}

table! {
    category_product (id_products, id_categories) {
        id_products -> Int4,
        id_categories -> Int4,
    }
}
