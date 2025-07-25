# what is the garbage collection ? 
-> *Garbage* is like the unrefrence object like any thing we declare in code and later we dont use it 

# Why does ownership exist?
-> the rust dont have any kinf of the garbage collecter 
-> the rust manage memory statically through the concept of the ownership 

# rules in the rust 

1- Each value in rust has single owner
 > here name has the ownership of the string abhi when name goes ot of the scope rust automatically free the memory 

2- Ownership is Transferred (Move)
 > when you assing one varibe to another varible it will not copy the value it will 
  assing the value to the assinged one 
 > here in the example if the name1 is assing to the naem2 not copied
 > If we try to print the name1 it will give the error as it dont have ownweship 

3- If owner goes out of the scope value is dropped 
 > Its like the class which can noot called out of the class

# stack vs heap memory 
> 