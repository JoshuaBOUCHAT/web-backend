use diesel::table;

table! {
    categories (id_category) {
        id_category -> Integer,
        name -> Text,
        description -> Text,
        super_category -> Nullable< Integer>
    }
}

table! {
    category_product (id_product, id_category) {
        id_product -> Integer,
        id_category -> Integer,
    }
}

table! {
    products (id_product) {
        id_product -> Integer,
        description -> Text,
        name -> Text,
        price -> Double,
        image_url -> Text,
        visible -> Integer
    }
}
table! {
    users (id_user) {
        id_user -> Integer,
        mail -> Text,
        phone_number -> Text,
        password_hash -> Text,
        date_creation -> Text, // SQLite stores NUMERIC loosely
        admin->Integer
    }
}

table! {
    orders (id_order) {
        id_order -> Integer,
        date_order -> Nullable<Text>,
        date_retrieve -> Nullable<Text>,
        id_user -> Integer,
    }
}

table! {
    order_product (id_order, id_product) {
        id_order -> Integer,
        id_product -> Integer,
        quantity -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(categories, category_product, products,);
diesel::allow_tables_to_appear_in_same_query!(users, orders, order_product, products,);
