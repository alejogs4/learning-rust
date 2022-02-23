mod restaurant;

use crate::restaurant::{front_house, Restaurant};

fn main() {
    let restaurant: Restaurant = Restaurant { employees: vec![] };
    let available_employee = front_house::get_available_employee(&restaurant);
    let employee_name = match available_employee {
        Some(employee) => &employee.name,
        None => "",
    };

    let got_employee_restaurant = match available_employee {
        Some(employee) => restaurant.mark_employee_unavailable(&mut employee),
        None => restaurant,
    };
}
