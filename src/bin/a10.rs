// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn show_message() {
    let val = 30;
    let is_big = val > 100;
    match is_big {
        true => println!("Its big"),
        false => println!("Its small"),
    };
}

fn main() {
    show_message()
}
