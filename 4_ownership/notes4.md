# What is Garbage Collection? 
-> Garbage refers to unreferenced objects that are declared in code but no longer used

# Why does Ownership Exist?
-> Rust doesn't have any kind of garbage collector
-> Rust manages memory statically through the concept of ownership

# Rules in Rust

1. Each Value in Rust Has a Single Owner
   > When a value is created, it's assigned to a variable (owner)
   > When the owner goes out of scope, Rust automatically frees the memory

2. Ownership is Transferred (Move)
   > When assigning one variable to another, the value is moved, not copied
   > Example: If name1 is assigned to name2, name1's ownership is transferred
   > Trying to use name1 after transfer will result in a compiler error

3. If Owner Goes Out of Scope, Value is Dropped
   > When a variable's scope ends, Rust automatically deallocates its memory

# Stack vs Heap Memory
> Stack memory is faster than heap memory
> Stack memory is automatically managed, heap memory needs manual management
> Stack memory is limited (MB), heap memory can be larger (GB)
> Stack memory is freed when function ends, heap memory persists until explicitly freed


# questions 
🔹 Conceptual Questions
1- What is ownership in Rust and why is it important?
 -> as rust do not have the garbage collector and rust manage memory through the concept of ownership 

2-What happens to a variable when it goes out of scope?
-> rust give the error

3-What is meant by a move in Rust?
 -> move in the rust means that we are transfreing the ownership of the varible 

3-Why can only one variable own a heap-allocated value at a time?
 -> you havve to expain this 
4-How does Rust ensure memory safety without a garbage collector?
-> By using the method of the ownership one varible has one value by defalut 
-> In every aspect of rust all thing are prettily predefind value 

5-What’s the difference between Copy and Move?
 -> move transfre the ownership from one varible to another varible 
 -> copy help the other varible to have same data as the desire varible 

5-Can two variables own the same heap data at the same time? Why or why not?
 -> yes, beacuse of the copy of the ownership 

6-What is Clone used for, and how is it different from a simple assignment?
 -> clone is used for the having the same ownership for the two varible.

7-Why are types like i32 or bool not moved but copied?
 -> explain this also

8-What happens when you pass a String to a function?
-> The fuction will have the ownership of string

🔹 Code Output / Compile Questions
Question 11:What will happen here?
fn main() {
    let a = String::from("Rust");
    let b = a;

    println!("{}", a);
}
A) Prints "Rust"
B) Compile-time error
C) Run-time error
D) Prints nothing
-> the ans is b

Question 12:
fn main() {
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);
}
What will be the output?
-> x:5 y:5

Question 13:
fn take_string(s: String) {
    println!("{}", s);
}

fn main() {
    let msg = String::from("Hello");
    take_string(msg);
    println!("{}", msg);
}
Will this compile? yes an this is called taking ownership

Question 14:
fn clone_string() {
    let s1 = String::from("Clone me");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
What will this print? s1 = Clone me, s2 = Clone me

Question 15:
fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;

    println!("{:?}", v2);
    println!("{:?}", v);
}
Does this compile?no this will not compile beacuse it do not clone 

🔹 Function Ownership Transfer
Question 16:

fn transfer(val: String) {
    println!("Got: {}", val);
}

fn main() {
    let v = String::from("Data");
    transfer(v);
    transfer(v); // What happens here?
}
-> this call the fuction fn transfer() and update  it value and print got: data

Question 17:
What’s the difference in behavior between these two?
// A
fn use_integer(n: i32) {
    println!("{}", n);
}

fn main() {
    let a = 10;
    use_integer(a);
    println!("{}", a);
}

// B
fn use_string(s: String) {
    println!("{}", s);
}

fn main() {
    let name = String::from("Ownership");
    use_string(name);
    println!("{}", name); // Error?
}

=> ya the code will give the error beacuse in teh first we are coping value form like updation and transfering the value 
   in the second one we are transfreing the ownership of the name to the s

🔹 Fix the Code
Question 18:
Fix the error in this code using .clone():
fn main() {
    let text = String::from("Clone it!");
    let copy = text;
    println!("{}", text);
}
=> in the second line we have to use let copy = text.clone();

Question 19:
How can you return ownership of a value from a function?
-> can you explain

Question 20:
What will be the final output?
fn give_back(s: String) -> String {
    s
}

fn main() {
    let val = String::from("Returned");
    let again = give_back(val);
    println!("{}", again);
}
=> explain this also 
