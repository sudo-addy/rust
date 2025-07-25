# Rust Programming - DSA Oriented Notes

## 📌 Variables & Data Types

- `let` is used to declare variables.
- Immutable by default, use `mut` to make them mutable.
- Strong static typing.

### Integer Types:

- Signed: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`
- Default: `i32`

### Float Types:

- `f32`, `f64` (default)

### Other Types:

- `bool`, `char`, `String`, `&str`, `tuple`, `array`

---

## 📌 Control Flow

- `if`, `else if`, `else`
- `loop`, `while`, `for`
- `match` (like `switch` in other languages)

---

## 📌 Functions

- Declared using `fn` keyword.
- Arguments are statically typed.
- Return values use `->`.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 📌 Ownership, Borrowing, and Lifetimes

- Unique to Rust for memory safety.
- Variables have **ownership** of values.
- Use `&` for **borrowing** references.
- Lifetimes ensure references are valid.

---

## 📌 Structs and Enums

- Custom data types.

### Struct:

```rust
struct Person {
    name: String,
    age: u32,
}
```

### Enum:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

---

## 📌 Pattern Matching

- `match`, `if let`, and `while let`

---

## 📌 Collections

- Vectors (`Vec<T>`) - growable arrays
- HashMap - key-value store

---

## 📌 Error Handling

- `Result<T, E>` and `Option<T>`
- `unwrap()`, `expect()` for fast prototyping
- Pattern matching recommended

---

## 📌 Modules and Crates

- Modules: Organize code with `mod`.
- Crates: External packages/libraries.

---

## 📌 Traits and Generics

- Traits: Define shared behavior.
- Generics: Write flexible and reusable code.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T { ... }
```

---

## 📌 DSA Focus Topics

- Arrays, Vectors
- Strings and Slicing
- Recursion
- HashMaps & Sets
- Stack, Queue (using Vec/VecDeque)
- Linked Lists (manual impl)
- Trees (binary, BST)
- Graphs (adj list/matrix)
- Sorting & Searching
- Dynamic Programming

---

## 🧠 Extra Tools

- `cargo` - Rust package manager
- `rustfmt` - code formatter
- `clippy` - linter
- `serde` - serialization library

---

## 📘 Practice Platforms

- [Leetcode (Rust supported)](https://leetcode.com)
- [Exercism](https://exercism.org/tracks/rust)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Codeforces / AtCoder] with Rust

---

## 📝 Notes

- Rust's safety guarantees make it ideal for DSA.
- Enforces best practices (immutability, types, memory safety).
- Excellent for building fast, reliable, low-level systems.

---

