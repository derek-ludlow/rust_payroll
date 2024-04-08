
// DATABASE_URL=postgres://username:password@localhost/payroll

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use schema::employees::dsl::*;

pub mod schema;
pub mod models;

use self::models::{Employee, NewEmployee};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_employee(conn: &PgConnection, name: String, salary: i32) -> Employee {
    use schema::employees;

    let new_employee = NewEmployee {
        name: name.clone(),
        salary,
    };

    diesel::insert_into(employees::table)
        .values(&new_employee)
        .get_result(conn)
        .expect("Error saving new employee")
}

fn read_employees(conn: &PgConnection) -> Vec<Employee> {
    use schema::employees::dsl::*;

    employees.load::<Employee>(conn)
        .expect("Error loading employees")
}

fn update_employee(conn: &PgConnection, id: i32, name: String, salary: i32) -> bool {
    use schema::employees::dsl::*;

    diesel::update(employees.find(id))
        .set((name.eq(name), salary.eq(salary)))
        .execute(conn)
        .is_ok()
}

fn delete_employee(conn: &PgConnection, id: i32) -> bool {
    use schema::employees::dsl::*;

    diesel::delete(employees.find(id))
        .execute(conn)
        .is_ok()
}

fn main() {
    let conn = establish_connection();

    // Example usage
    let new_employee = create_employee(&conn, "Alice".to_string(), 50000);
    println!("Created employee: {:?}", new_employee);

    let employees = read_employees(&conn);
    println!("All employees: {:?}", employees);

    let success = update_employee(&conn, new_employee.id, "Alice Smith".to_string(), 55000);
    println!("Update successful: {:?}", success);

    let success = delete_employee(&conn, new_employee.id);
    println!("Deletion successful: {:?}", success);
}
