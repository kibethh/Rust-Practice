// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let value = "my value Yellow";
    match value {
        "some value" => println!("Not my value"),
        "my value" => println!("This is my value"),
        _ => println!("Some other value"),
    }
}
