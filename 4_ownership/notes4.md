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

# Scope
> In rust scope is the block of code where one varible is valid
> After the block of code the block of code become unusable like the out of the scope

# questions 
✅ Q1: What is ownership in Rust and why is it important?
Your Answer (Good start):

Rust does not have a garbage collector and manages memory through ownership.

Expanded Answer:
Ownership is Rust’s unique system for managing memory without a garbage collector. Every value in Rust has a single owner, which is the variable that holds it. When the owner goes out of scope, the value is automatically dropped (freed). This avoids memory leaks and makes memory usage safe and efficient at compile time.

✅ Q2: What happens when a variable goes out of scope?
Your Answer:

rust give the error

Correction:

It doesn’t give an error. Rust will automatically free (drop) the memory of the variable when it goes out of scope.

Example:

rust
Copy
Edit
{
    let s = String::from("hello");
    // s is valid here
}
// s goes out of scope and is dropped — no error
✅ Q3: What is meant by a move in Rust?
Your Answer:

Move means we are transferring the ownership of the variable.

Perfect.
A move happens when ownership is transferred from one variable to another. After the move, the original variable becomes invalid (for heap data).

🔶 Q4: Why can only one variable own heap-allocated data at a time?
Your Question: "You have to explain this."

Explanation:
Heap-allocated data (like String, Vec, etc.) doesn't automatically get duplicated when assigned to another variable. If multiple variables had ownership, it would be hard to know who should clean up the memory. Rust prevents this by allowing only one owner. This avoids double frees and makes memory management predictable and safe.

✅ Q5: How does Rust ensure memory safety without a garbage collector?
Your Idea:

By using the method of the ownership, one variable has one value by default.

Expanded Explanation:
Rust uses the ownership system, along with borrowing and lifetimes, to track who owns what. At compile time, Rust can check memory access and prevent use-after-free, data races, or double frees — all without a garbage collector.

🔶 Q6: What’s the difference between Copy and Move?
Your Answer:

Move transfers ownership. Copy allows two variables to have the same value.

Correct! But let me add:

Types like integers (i32, bool, char, etc.) implement the Copy trait. This means assigning them makes a full duplicate, not a move.

Complex types (like String, Vec) are moved by default unless .clone() is used.

❌ Q7: Can two variables own the same heap data at the same time?
Your Answer:

Yes, because of the copy of ownership.

Correction:

❌ No! Only one variable can own heap data.
If you want two variables to have the same data, you must use .clone(), which performs a deep copy, creating a separate copy in memory.

✅ Q8: What is Clone used for?
.clone() makes a deep copy of heap data. This is different from assignment which causes a move.

🔶 Q9: Why are types like i32 or bool not moved but copied?
Your Question: "Explain this."

Answer:
These are small, simple types stored entirely on the stack. They implement the Copy trait, which allows duplication without worrying about heap memory or ownership issues. Because copying them is cheap and safe, Rust does it automatically.

✅ Q10: What happens when you pass a String to a function?
Ownership is transferred to the function, and the original variable can no longer be used unless returned.

✅ Code-Based Questions
🔹 Q11:
rust
Copy
Edit
fn main() {
    let a = String::from("Rust");
    let b = a;

    println!("{}", a);
}
✅ Correct Answer: B - Compile-time error
Because a was moved to b.

🔹 Q12:
rust
Copy
Edit
fn main() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
}
✅ Correct — These are Copy types.

🔹 Q13:
rust
Copy
Edit
let msg = String::from("Hello");
take_string(msg);
println!("{}", msg);
❌ Incorrect. It will not compile because msg is moved into the function. After the call, msg is no longer valid unless it’s returned.

🔹 Q14:
rust
Copy
Edit
let s1 = String::from("Clone me");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
✅ This will compile and output both strings.

🔹 Q15:
rust
Copy
Edit
let v = vec![1, 2, 3];
let v2 = v;
println!("{:?}", v2);
println!("{:?}", v);
❌ This will NOT compile. v was moved to v2.

🔹 Q16:
rust
Copy
Edit
fn transfer(val: String) { println!("Got: {}", val); }
let v = String::from("Data");
transfer(v);
transfer(v); // ❌ Error: ownership already moved
🔹 Q17:
A: Uses Copy type (i32), so no error.
B: String is moved; second println! causes error.

🔹 Q18: Fix the clone
rust
Copy
Edit
let copy = text.clone(); // ✅ Fixed
🔹 Q19: How to return ownership?
Example:

rust
Copy
Edit
fn give_back(s: String) -> String {
    s
}

fn main() {
    let val = String::from("Returned");
    let again = give_back(val);
    println!("{}", again); // ✅ Ownership returned
}