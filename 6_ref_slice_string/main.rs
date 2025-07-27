// Code 1

fn main() {
    let s = String::from("hello");
    let _r = &s; // Added underscore to suppress unused variable warning

    let arr = [1, 2, 3, 4];
    let _slice = &arr[1..3]; // Added underscore to suppress unused variable warning

}