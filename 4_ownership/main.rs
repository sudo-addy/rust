fn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); 

fn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ikfn main() {
// Rule 1: Each value in Rust has a single owner
    let name = String::from("Abhi");
    println!("name = {}", name);

// Rule 2: Ownership is Transferred (Move)
    let name1 = String::from("abhi");
    let name2 = name1;  // ownership moves from name1 to name2

    // println!("name1 = {}", name1);  // Error: value borrowed after move
    println!("name2 = {}", name2);     // OK: name2 owns the string now

    // Note: The original value name1 can no longer be used
    // because its ownership was transferred to name2

// Rule 3: If Owner Goes Out of Scope, Value is Dropped
    let s = String::from("Hello");    // 1. Memory is allocated

// 2. Memory is automatically freed here (s is "dropped")


// Stack vs heap memory
let x = 5;
    let y = x; // x is still usable because integers are Copy types

    let a = String::from("Hi");
    let b = a; // a is now invalid, ownership moved to b'

// Clone(copying the owneership )

let s1 = String::from("Data");
let s2 = s1.clone(); // Now both own separate memory



 }

 fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn host() {
    let msg = String::from("Rust is safe");
    take_ownership(msg); // Ownership moved
    // println!("{}", msg);  Error
}

let namm = String::from("abhi"); hey this ik