fn main(){
    // if-else loop 

    let i :i64= 1000000000000; 

    if i< 5{
        println!("The Number is Less than 5 ");
    }else if i == 5 {
        println!(" the Number is Equal");
    }else{
        println!("The number is grarter than 5");
    }

    // Match condition 
    let nig =23 ; 
    match nig{
    1=> println!("one"),
    2=> println!("two"),
    3=> println!("three"),
    4=> println!("four"),
    _ => println!("Something else"),
    }

    // Looop infinite 
    let mut count =0;

    loop {
        count += 1;
        println!("the count is{}" ,count);
        if count == 100 {
        break; 
        }
    }
    println!("the count is {}", count);
}