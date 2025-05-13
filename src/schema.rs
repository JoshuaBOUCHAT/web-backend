use diesel::table;

table! {
    categories (id_categories) {
        id_categories -> Integer,
        name -> Text,
        description -> Text,
    }
}

table! {
    category_product (id_products, id_categories) {
        id_products -> Integer,
        id_categories -> Integer,
    }
}

table! {
    products (id_products) {
        id_products -> Integer,
        description -> Text,
        name -> Text,
        price -> Double,
        image_url -> Nullable<Text>,
    }
}
table! {
    users (id_users) {
        id_users -> Integer,
        mail -> Text,
        phone_number -> Text,
        password_hash -> Text,
        date_creation -> Text, // SQLite stores NUMERIC loosely
        admin->Integer
    }
}

table! {
    orders (id_orders) {
        id_orders -> Integer,
        date_order -> Text,
        date_retrieve -> Text,
        id_users -> Integer,
    }
}

table! {
    order_product (id_orders, id_products) {
        id_orders -> Integer,
        id_products -> Integer,
        nombre -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(categories, category_product, products,);
diesel::allow_tables_to_appear_in_same_query!(
    users,
    orders,
    order_product,
    products, // assuming you already have a `products` table defined
);
