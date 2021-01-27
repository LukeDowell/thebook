/// # Ownership
/// Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees
/// without needing a garbage collector. Therefore, it's important to understand how ownership works
/// in Rust. In this chapter, we'll talk about ownership as well as several related features: borrowing,
/// slices, and how Rust lays data out in memory.
pub fn run() {
    println!("Chapter 4: Ownership");

    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
    } // this scope is ended, s is no longer valid

    // This String type is allocated on the heap and as such is able to store an
    // amount of text that is unknown to us at compile time.
    let mut s = String::from("hello");

    // This kind of string can be mutated
    s.push_str(", world!");
    println!("{}", s);

    // & is a reference, or a pointer, to an existing value
    // They allow us to refer to the value without taking ownership of it
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}", s1); // s1 is still valid!

    // Mutable reference
    let mut s2 = String::from("mutable string");
    let len2 = legal_calculate_length(&mut s2);

    // You can only have one mutable reference to a particular piece of data in scope at a time
    let r1 = &mut s2;
    // This will fail!
    // let r2 = &mut s2;
}

// Having a reference as a function param is called borrowing
// The implication is that when the function is done, the parameter must be returned to the caller
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Attempting to mutate a borrowed parameter won't work
// fn bad_calculate_length(s: &String) -> usize {
//     s.push("illegal!")
// }

// Unless we mark the reference as mutable
fn legal_calculate_length(s: &mut String) -> usize {
    s.push_str("addendum!");
    s.len()
}

// Rust prevents dangling pointers, or references to locations in memory that no longer contain data
// The following code would throw an "expected named lifetime parameter" error at compilation time
// fn dangle() -> &String {
//     Once this function exits, s is dropped, and the returned pointer is invalid!
//     let s = String::from("hi!");
//     &s
// } In this case, the solution would be to simply return the data, not a pointer to the data