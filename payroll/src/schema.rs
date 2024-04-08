extern crate diesel;

table! {
    employees (id) {
        id -> Int4,
        name -> Varchar,
        salary -> Int4,
    }
}
