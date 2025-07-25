# Data-type in the rust 
->Rust is the statically typed language the type of each varible must be know at compile time

## Scalar type 
-The scalar hold only one type of the value
-Singed number store the positive and the negtive value
-unsinged number store only positive number.

## Compund type 
- this type hold many value in the one like arry tuple 
- The


## 🧠 Topic: Data Types in Rust

### 1. Scalar Types
- `i32`, `u32`, `f64`, `bool`, `char` — hold a single value
- `i` = signed (can be negative), `u` = unsigned
- `f32` and `f64` for floating points (default: `f64`)
- `char` stores Unicode characters, not just ASCII

### 2. Compound Types
- Tuple: `(i32, f64, char)` — fixed-size collection of multiple values
- Access with `.0`, `.1`, etc.
- Array: `[i32; 5]` — fixed-length collection of same-type items
- Access with index like `array[0]`

### 3. Example
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let a = [1, 2, 3, 4, 5];
    println!("First element: {}", a[0]);
}


# Quesion 

    1. What is the difference between `i32` and `u32`?
    -> The i32 here tell the vaue we stored here is singed ie the value can be positive or negative ,This store the 32 bits of the memory 
    -> The u32 here tell the value is unsinged and we can store only the positive numver 

    2. Why does Rust default to `i32` and `f64` instead of `i64` or `f32`?
    -> Genrally the float value are always big so rust autimatically assing it that value on the other side the sined number genrally we use very less value.

    3. How is a tuple different from an array in Rust?
     -> the tuple are of the fix size as the arry like untill we use the mut
     ->     
    4. Can you store different types in an array?
    -> no we cant store the diff type of vaue in one arrya the arry define the  group of the same data type

    5. What happens if you try to access an index out of range in a Rust array?
    -> it will  give the our of teh index error 



