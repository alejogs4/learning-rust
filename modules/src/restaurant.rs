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

    pub fn mark_employee_unavailable(self: Self, employee: &mut Employee) -> Restaurant {
        let restaurant_without_employee: Vec<&Employee> = self
            .employees
            .iter()
            .filter(|current_employee| current_employee.name != employee.name)
            .collect();

        if restaurant_without_employee.len() != self.employees.len() {
            // Research if there is a way to change without mutate it
            employee.available = false;
        }
        self
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
