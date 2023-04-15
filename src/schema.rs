// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Int4,
        code -> Varchar,
        url -> Varchar,
        questions -> Array<Nullable<Text>>,
        upvotes -> Array<Nullable<Int4>>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    posts,
);
