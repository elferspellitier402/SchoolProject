use std::fmt;

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}

fn main() {
    let student = Student {
        name: String::from("Alice"),
        age: 18,
    };

    println!("{}", student);
}
