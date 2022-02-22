mod restaurant {
    enum PlaceWork {
        Front,
        Back,
    }
    struct Employee {
        pub name: String,
        pub salary: i32,
        pub available: bool,
        pub work_place: PlaceWork,
    }
    pub struct Restaurant {
        pub employees: Vec<Employee>,
    }

    impl Restaurant {
        fn create_restaurant(employees: Vec<Employee>) -> Restaurant {
            Restaurant { employees }
        }

        pub fn mark_employee_unavailable(self: Self, employee: &Employee) -> Restaurant {
            let mut restaurant_without_employee: Vec<&Employee> = self
                .employees
                .iter()
                .filter(|current_employee| current_employee.name != employee.name)
                .collect();

            if restaurant_without_employee.len() != self.employees.len() {
                let unavailable_employee = Employee {
                    available: false,
                    name: employee.name.clone(),
                    ..(*employee)
                };
                restaurant_without_employee.append(&mut vec![unavailable_employee]),
                return Restaurant::create_restaurant(
                    restaurant_without_employee
                );
            }
            return self;
        }
    }

    pub mod front_house {
        use super::{Employee, PlaceWork, Restaurant};

        pub fn get_available_employee(restaurant: &Restaurant) -> Option<&Employee> {
            let available_employee =
                restaurant
                    .employees
                    .iter()
                    .find(|employee| match employee.work_place {
                        PlaceWork::Front => employee.available,
                        PlaceWork::Back => false,
                    });
            available_employee
        }
    }
}

use crate::restaurant::{front_house, Restaurant};

fn main() {
    let restaurant: Restaurant = Restaurant {employees: vec![]};
    let available_employee = front_house::get_available_employee(&restaurant);
    let employee_name = match available_employee {
        Some(employee) => employee.name,
        None => String::from("_")
    };

    let got_employee_restaurant = match available_employee {
        Some(employee) => restaurant.mark_employee_unavailable(employee),
        None => restaurant
    };
    
}
