use std::io; 

fn main(){

    let mut x = String::new(); 

    println!("what is the value of the x"); 

    io::stdin()
    .read_line(&mut x)
    .expect("Failed to read the value"); 
 

    println!("the value of the x is {} ", x );


    let mut y = String::new()

    io::stdin()
    .read line(&mut x)
    
}