// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Tickets::Backstage(20.3, String::from("Humphrey")),
        Tickets::Vip(30.5, String::from("Kibet")),
        Tickets::Standard(25.0),
    ];

    for ticket in tickets {
        match ticket {
            Tickets::Backstage(price, holder) => {
                println!("Price: {:?}, Holder: {:?}", price, holder)
            }
            Tickets::Vip(price, holder) => println!("Price: {:?}, Holder: {:?}", price, holder),
            Tickets::Standard(price) => println!("Price: {:?}", price),
            _ => (),
        }
    }
}
