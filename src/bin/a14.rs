// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

impl Person {
    fn print(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Favorite Color: {}", self.favorite_color);
    }
}

fn print_name_color(name: &str, color: &str) {
    println!("Name: {}", name);
    println!("Color: {}", color);
}

fn main() {
    let people = vec![
        Person {
            age: 20,
            name: String::from("hkk"),
            favorite_color: "white".to_owned(),
        },
        Person {
            age: 09,
            name: ("kkh").to_owned(),
            favorite_color: "white".to_owned(),
        },
        Person {
            age: 25,
            name: ("bkk").to_owned(),
            favorite_color: "white".to_owned(),
        },
    ];

    for person in people {
        //Testing
        // person.print();

        if person.age < 10 {
            print_name_color(&person.name, &person.favorite_color);
        }
    }
}
