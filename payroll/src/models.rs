use diesel::{Queryable, Insertable};

#[derive(Queryable, Debug)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub salary: i32,
}

#[derive(Insertable)]
#[table_name = "employees"]
pub struct NewEmployee {
    pub name: String,
    pub salary: i32,
}
