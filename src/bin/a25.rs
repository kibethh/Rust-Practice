// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn perimeter(&self);
}

struct Square {
    length: f64,
}
impl Perimeter for Square {
    fn perimeter(&self) {
        let result = self.length * 4.0;
        println!("Perimeter of a square is: {:?}", result);
    }
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Perimeter for Triangle {
    fn perimeter(&self) {
        let result = self.a + self.b + self.c;
        println!("Perimeter of a triangle is: {:?}", result);
    }
}

fn perimeter(shape: impl Perimeter) {
    shape.perimeter();
}

fn main() {
    perimeter(Triangle {
        a: 23.1,
        b: 90.2,
        c: 14.9,
    });
    perimeter(Square { length: 7.6 });
}
