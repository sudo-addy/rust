fn main() {
    // Scalar types  

    // Integer signed and unsigned 
    let x: i8 = 1;  // this is the signed integer
    let y: u8 = 1;  // this is the unsigned integer

    // Float type
    let v: f32 = 1.3;
    
    // Boolean type 
    let is_hungry = true; 

    // Char type 
    let character = 'A';
    let name = "abhijit";

    println!("x={}, y={}, v={}, is_hungry={}, char={}, name={}", 
             x, y, v, is_hungry, character, name);
    
    // Compound type 

    // tuple 
    let tup = (20, 34.3, 'a');
    let (a, b, c) = tup;  // Using different variable names to avoid shadowing
    println!("Tuple values: a={}, b={}, c={}", a, b, c);

    //Array
    let arr = [12,21,32];
    println!("{}",arr[0]);
    
}