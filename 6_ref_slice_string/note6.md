# Refrence code1 
 -> A refrence in rust ler you refer some value without taking the ownership
 -> In the code we are borrowing the value without taking the ownership

# Slice
 -> A slice is a way of the viewing the data without takin the ownership 

# Mutable vs immutable refrence 
 ->❌ You can't have both mutable and immutable references at the same time.
✅ At a time: either many immutable references, or one mutable.

# Practice Questions on References and Slices

## Section 1: Basic References
**Q1. What will this code print?**
```rust
let x = 10;
let r = &x;
println!("{}", r);
```
- a) 10
- b) Compiler error
- c) Address of x
- d) Undefined behavior
=> a

**Q2. What is the type of `r` in the above code?**
 => In above code r is the refence which borrow value from the x

**Q3. Explain the difference between `let x = 10;` and `let ref x = 10;`?**
 =>let x = 10; creates an i32 value on the stack.
 =>let ref x = 10; also creates x on the stack, but x is a reference to that value.
 =>So it's equivalent to: let x = &10;

## Section 2: Mutable References
**Q4. Identify the error in this code:**
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
```
 => I know the answe but i dont know how to write it(In rust the there should be only one ref allowing to edit mut the value )

**Q5. What will happen here?**
```rust
let mut x = 5;
let r1 = &x;
let r2 = &x;
let r3 = &mut x;
println!("{}, {}, {}", r1, r2, r3);
```
=> This will give the error i thinks 

## Section 3: Slices
**Q6. What is the output?**
```rust
let s = String::from("abcdef");
let slice = &s[1..4];
println!("{}", slice);
```
=>bcd

**Q7. True or False: A slice can outlive the data it refers to.** => False idk but as i know teh slice is the function whihc is used to show the data without taking the ownership 

**Q8. Which of the following is valid?**
- a) `let s: &str = &String::from("hello");`
- b) `let s: &str = "hello".to_string();`
- c) `let s: String = "hello";`
- d) `let s: String = "hello".to_string();`

=>a & d
## Section 4: String Operations
**Q9. What's the output?**
```rust
let mut s = String::from("foo");
s.push_str("bar");
println!("{}", s);
```
=>foo bar (you dont tell me about the fucntion but i am guessing this answeer)

**Q10. What is the difference between `String::from("hello")` and `"hello"`?**
=> The first one is like we are giving the ownership to the varible and second one i thinks it assing the value 

## Section 5: Error Debugging
**Q11. Identify and fix the error:**
```rust
fn main() {
    let mut s = String::from("hello");
    let slice = &s[0..2];
    s.push_str(" world");
    println!("{}", slice);
}
```
```rust
=>fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    let slice = &s[0..2];
    println!("{}", slice);
}
```


**Q12. Is this valid? Why or why not?**
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);
```
=> This is not valid, Beacure of the ownership transfer we are transerting the ownership of the s1 ro the s2


## Section 6: Theory Questions
1. Why does Rust restrict having multiple mutable references?
 => To preven the data races at the compile time ,rust ensure that only one peice of code will run at one time 

2. When would you prefer using a slice instead of passing the whole object?
 => I will do this only when i need to accest the data without taking the ownership 

3. What's the difference between `&String` and `&str`?&
 => the string has all the data assing to it 
 => str is use to display teh data of it 

4. Can you mutate data through a reference? Under what conditions?
=> Yes, but only when the varible is mutable like mut y if it is immutable i can mutate it 



