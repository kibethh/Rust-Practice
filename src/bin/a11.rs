// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    id: i32,
    quantity: i32,
}
fn display_quantity(item: &Item) {
    println!("Quantity:{:?}", item.quantity);
}

fn display_id(item: &Item) {
    println!("ID:{:?}", item.id);
}
fn main() {
    let grocery_item = Item { id: 1, quantity: 2 };
    display_quantity(&grocery_item);
    display_id(&grocery_item);
}
