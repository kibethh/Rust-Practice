// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Strawberry,
    Raspberry,
    Vanilla,
}

struct Drink {
    flavor: Flavors,
    fluid_ounce: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavors::Strawberry => println!("Strawberry"),
        Flavors::Raspberry => println!("Raspberry"),
        Flavors::Vanilla => println!("Vanilla"),
    };
    println!("ounces are: {:?}", drink.fluid_ounce)
}

fn main() {
    let drink = Drink {
        flavor: Flavors::Raspberry,
        fluid_ounce: 20.7,
    };
    print_drink(drink);
}
