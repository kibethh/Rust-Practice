// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Assignment {
    name: String,
    locker_assign: Option<i32>,
}

fn main() {
    let stud_assign = vec![
        Assignment {
            name: "kibethh".to_owned(),
            locker_assign: Some(20),
        },
        Assignment {
            name: "kipr".to_owned(),
            locker_assign: None,
        },
    ];

    for assign in stud_assign {
        match assign.locker_assign {
            Some(num) => println!("{}", num),
            None => println!("No locker assignment"),
        }
    }
}
