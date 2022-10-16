// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_numbers = vec![10, 20, 30, 40];
    println!("There are: {} elements!", my_numbers.len());
    for num in my_numbers {
        if num == 30 {
            println!("thirty");
        } else {
            println!("{}", num);
        };
        println!("In match!!!!!!");
        match num {
            10 => println!("{}", 10),
            20 => println!("{}", 20),
            30 => println!("{}", "thirty"),
            40 => println!("{}", 40),
            _ => println!("Oops!"),
        }
    }
}
