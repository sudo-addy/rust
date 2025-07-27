# Lifetime 
 -> Rust is all about the  memory safety without the garbage collector, espicassly from the funciton 
 - Rust make sure that "the refrence is never use after the value is dropped"

*- here the life time comes it help the compiler to check 
  1- How long each refrence is valid 
  2- Whether it is safe to use 


# When we are using the lifetime 
 -> A function having the multiple refrence 
 -> A function return a refrence
 -> Where Rust cant interfare the correct life time 

 # program 1

 - <'a>- lifetime annotation 


# Lifetime Elision Rule