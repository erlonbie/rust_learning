// @generated automatically by Diesel CLI.

diesel::table! {
    pokemon (id) {
        id -> Nullable<Integer>,
        name -> Text,
        height -> Integer,
        weight -> Integer,
    }
}
