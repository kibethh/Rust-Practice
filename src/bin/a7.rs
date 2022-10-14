// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Blue,
    Red,
    Green,
    Yellow,
}

fn print_color_name(color: Colors) {
    let fav_color = color;
    match fav_color {
        Colors::Blue => println!("Chelsea"),
        Colors::Red => println!("Bayern"),
        Colors::Yellow => println!("Dortmund"),
        Colors::Green => println!("Celtic"),
    }
}

fn main() {
    print_color_name(Colors::Blue);
}
