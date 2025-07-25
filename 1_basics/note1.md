***IMP*** your code should be side by side by side to the code 
# fn main()
 - Here we are learing about how can we print something in the rust

# Declearation of the varible 

 - let is a keyword use to declear the varible
 - {} this is called the place holder which can be use as like
 - *imp* we can redeclear the varible 

'''pintln!("my name is {} and age{} my hobbies are {}", name , age, hobbie)

# Mutable varible 

 - By default we cant change the vallue of the varible in the rust
 - make the varible like we can update it value like we use for our age or any thing which can inc by time 


# Summary
## 🟢 Declaration
- `let x = 5;` → immutable by default.
- `let mut x = 10;` → mutable variable.

## 🔤 Types
- `i32`, `u32`, `f64`, `bool`, `&str` (string slice)
- You can annotate: `let a: i32 = 42;`

## 🔁 Shadowing
- You can re-use the same variable name:
  ```rust
  let x = 1;
  let x = x + 1;

# questions

1-What is the default behavior of a variable declared with let in Rust?
-> They are not mutable as the default.

2-How do you make a variable mutable in Rust?
-> you have to use the mut like "let mut y = 19;.

3-What is shadowing in Rust, and how is it different from mutability?
-> Shadowing is noting but redeclartion of the same varible to update it.

4-Can you assign a new value to an immutable variable? What happens if you try?
->we can but we have to use method of shadowing in rust,other wise the compiler will give the error.

5-What is the purpose of strong static typing in Rust?
->Catch errors early at compile-time, before running the code.
->Optimize performance since the compiler knows exactly what types are being used.
->Ensure safety, especially in memory management and concurrency.
->Makes the code more predictable and reliable.

6-How would you declare a variable of type f64 with a value 3.1415?
-> let a: f64 = 3.141;

7-Why might Rust prefer immutability by default?
-> Its more like the global varible like value of pi which make it value simple to use and we cant change it and the program will look clean and more efficinet

8-What does the following code do?
let x = 5;
let x = x + 1;
println!("{}", x);
How can you declare a string slice in Rust? ->6

9-What is the difference between let mut x = 5; and let x = 5; x = 6;?
->in the first case we are making the value of mut mutable.
->in the second case we are making value of x immutable.
->in the third look like we are assing the value for the mutable value of the x 

