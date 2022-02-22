struct Point(u32, u32, u32);

impl Point {
    fn initial() -> Point {
        return Point(0, 0, 0);
    }

    fn equal(&self, another: &Point) -> bool {
        return self.0 == another.0 && self.1 == another.1 && self.2 == another.2;
    }
}

struct Color(u32, u32, u32);

struct Person {
    name: String,
    lastname: String,
    age: i16,
}

fn build_person(name: String, lastname: String, age: i16) -> Person {
    return Person {
        name,
        lastname,
        age,
    };
}

fn update_person_name(person: Person, new_name: String) -> Person {
    return Person {
        name: new_name,
        ..person
    };
}

fn main() {
    let first_person = build_person(String::from("Alejandro"), String::from("Garcia"), 22);
    println!(
        "{} {} {}",
        first_person.name, first_person.lastname, first_person.age
    );
    let updated_person = update_person_name(first_person, String::from("Alejandra"));
    println!(
        "{} {} {}",
        updated_person.name, updated_person.lastname, updated_person.age
    );

    let color: Color = Color(255, 255, 255);
    let point: Point = Point(255, 255, 255);
    let second_point = Point(255, 255, 256);
    let initial_position = Point::initial();

    print_color(color);
    println!("are the same = {}", point.equal(&initial_position));
}

fn print_color(color: Color) {
    println!("{} {} {}", color.0, color.1, color.2);
}
