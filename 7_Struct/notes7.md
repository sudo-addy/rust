# THE STRUCT
=> The struct is the basic syntax which is used to store Information 
=> The struct has readable data, groups related data, and builds clean and readable code 

```rust
struct Person {
    name: String,
    age: u32,
    height: f32
}
```

# Struct Types 

## 1-Regular struct
```rust
struct Book {
    title: String,
    pages: u32
}
```
## 2-tuple struct
```rust
struct Color(u8, u8, u8);
```

# IMPL (implementing method on the struck ) 

# Question 

🧠 Part A: MCQs – Choose the Correct Answer
1-What does a struct represent in Rust?
a) A pointer
b) A function
c) A custom data type
d) A loop construct
=> a

2-Which keyword is used to define a struct in Rust?
a) type
b) class
c) struct
d) define
=> c

3-Which of the following is a tuple struct?
a) struct Point { x: i32, y: i32 }
b) struct Point(i32, i32);
c) struct Point;
d) let point = (10, 20);
=>a,b(i think c is also but it empty )

4-How do you access a field of a struct instance?
a) object->field
b) object::field
c) object.field
d) object[field]
=>b

5-What will this print?
struct Color(u8, u8, u8);
fn main() {
    let red = Color(255, 0, 0);
    println!("{}", red.0);
}
a) red
b) 255
c) 0
d) Compilation error
=>b

🧠 Part B: True or False
1-struct in Rust can have methods like classes.- true

2-You can mutate struct fields if the struct instance is let mut.- true 

3-impl blocks are used to implement behavior for structs. - true 

4-Unit-like structs can hold multiple fields. true

5-Rust structs support inheritance.you dont teach me inheritance 

🧠 Part C: Code Output / Prediction
1-What will be the output?

struct Animal {
    name: String,
    legs: u8,
}

fn main() {
    let a = Animal {
        name: String::from("Dog"),
        legs: 4,
    };

    println!("{} has {} legs", a.name, a.legs);
}
=>Dog had 4 legs

2-Predict what happens here:
fn main() {
    let m = Marker;
    println!("Marker created!");
}
Will this compile?

=> yes,This will comile value of m is string marker and to print the marker created 
  if no then the rust does want any unassinged thing while comping 

struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let b1 = Book {
        title: "Rust".to_string(),
        pages: 300,
    };
    
    let b2 = Book {
        title: String::from("Clone Book"),
        ..b1
    };
    
    println!("{}", b1.title);
}
=> this will give the error 

