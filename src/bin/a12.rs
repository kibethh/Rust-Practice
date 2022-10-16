// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColors {
    Brown,
    Blue,
}

impl BoxColors {
    fn print(&self) {
        match &self {
            BoxColors::Brown => println!("Brown"),
            BoxColors::Blue => println!("Blue"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    length: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("length: {:?}", self.length);
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColors,
}

impl Box {
    fn new(dimensions: Dimensions, color: BoxColors, weight: f64) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }
    fn print_characteristics(&self) {
        self.dimensions.print();
        self.color.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let sm_dimenstions = Dimensions {
        width: 20.4,
        height: 50.4,
        length: 55.4,
    };
    let new_box = Box::new(sm_dimenstions, BoxColors::Brown, 34.5);
    new_box.print_characteristics();
}
