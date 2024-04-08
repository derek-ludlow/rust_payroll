use std::collections::HashMap;

// Employee struct representing an employee
#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
    salary: i32,
}

impl Employee {
    // Constructor method to create a new employee
    fn new(id: i32, name: String, salary: i32) -> Self {
        Employee { id, name, salary }
    }
}

// Payroll struct representing a payroll system
struct Payroll {
    employees: HashMap<i32, Employee>,
    next_id: i32,
}

impl Payroll {
    // Constructor method to create a new empty payroll system
    fn new() -> Self {
        Payroll {
            employees: HashMap::new(),
            next_id: 1,
        }
    }

    // Method to add a new employee to the payroll
    fn add_employee(&mut self, name: String, salary: i32) -> i32 {
        let id = self.next_id;
        let employee = Employee::new(id, name, salary);
        self.employees.insert(id, employee);
        self.next_id += 1;
        id
    }

    // Method to retrieve an employee by ID
    // fn get_employee(&self, id: i32) -> Option<&Employee> {
    //     self.employees.get(&id)
    // }

    // Method to update an employee's information
    fn update_employee(&mut self, id: i32, name: String, salary: i32) -> bool {
        if let Some(employee) = self.employees.get_mut(&id) {
            employee.name = name;
            employee.salary = salary;
            true
        } else {
            false
        }
    }

    // Method to remove an employee from the payroll
    fn remove_employee(&mut self, id: i32) -> Option<Employee> {
        self.employees.remove(&id)
    }

    // Method to retrieve all employees in the payroll
    fn get_all_employees(&self) -> Vec<&Employee> {
        self.employees.values().collect()
    }
}

fn main() {
    let mut payroll = Payroll::new();

    // Add some employees to the payroll
    let id1 = payroll.add_employee("Alice Smith".to_string(), 50000);
    let id2 = payroll.add_employee("Bob Frederic".to_string(), 60000);
    let id3 = payroll.add_employee("Shaun Johnson".to_string(), 75000);
    let id4 = payroll.add_employee("Joshua".to_string(), 82000);

    // Print details of all employees
    println!("All employees:");
    for employee in payroll.get_all_employees() {
        println!("{:?}", employee);
    }

    // Update an employee's information
    println!("Updating employee with ID {}: {:?}", id1, payroll.update_employee(id1, "Alice Jones".to_string(), 50000));

    println!("Updating employee with ID {}: {:?}", id4, payroll.update_employee(id4, "Joshua Jacobs".to_string(), 90000));

    // Print details of all employees after update
    println!("All employees after update:");
    for employee in payroll.get_all_employees() {
        println!("{:?}", employee);
    }

    // Remove an employee from the payroll
    println!("Removing employee with ID {}: {:?}", id2, payroll.remove_employee(id2));

    // Print details of all employees after removal
    println!("All employees after removal:");
    for employee in payroll.get_all_employees() {
        println!("{:?}", employee);
    }
}
