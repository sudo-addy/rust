struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut person1 = Person {
        name: String::from("abhi"),
        age: 20,
    };
    println!("{} is {} years old", person1.name, person1.age);

    
// MUTABLE STRUCKS - just adding the mut in bw the ler and person 1

person1.age += 1;

   println!("{} is {} years old", person1.name, person1.age);

}

/// Questioin 1 
// 🧠 Part D: Coding Practice
// imp= you dont taught me about loops 
// 14. Create a struct Student with fields: name: String, marks: u32.
// Add a method is_pass() that returns true if marks ≥ 40.
// => 
struct Student {
    name: String,
    marks: u32,
}

impl Student {
    fn is_pass(&self) -> bool {
        self.marks >= 40
    }
}

fn mains() {
    let student1 = Student {
        name: String::from("Abhijit"),
        marks: 40,
    };

    println!(
        "{} scored {} and has {} the exam.",
        student1.name,
        student1.marks,
        if student1.is_pass() { "passed" } else { "failed" }
    );
}

// 15. Create a Rectangle struct with length and width.
// Add a method area() to calculate area.

// Create two rectangle instances and compare their areas.

// 16. Create a struct Laptop with fields brand, price, and ram_gb.
// Add a method is_gaming() → returns true if ram_gb >= 16.

