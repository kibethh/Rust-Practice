// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    store.insert("chairs", 5);
    store.insert("beds", 3);
    store.insert("tables", 2);
    store.insert("couches", 0);

    let mut total_number = 0;
    for (item, number) in store.iter() {
        total_number += number;
        let stock_count = if number == &0 {
            "out of stock".to_owned()
        } else {
            // Just like println but doesn't print instantly. Saves it into a string
            format!("{:?}", number)
        };

        println!("Item: {:?} stock={:?}", item, stock_count);

        match number {
            0 => println!("{:?} are out of stock!", item),
            _ => println!("{:?} are: {:?} in store", item, number),
        };
    }
    println!("total items: {:?}", total_number);
}
