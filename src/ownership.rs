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
}
