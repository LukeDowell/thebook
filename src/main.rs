use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Calling something with a ! after is a macro, not a function
    println!("Guess the number!");

    // rand:Rng is a trait, and rand::thread_rng() provides an implementation of that trait
    let secret_number = rand::thread_rng().gen_range( 0..101);

    // creates an infinite loop
    loop {
        // :: indicates that new is an associated function of the String type. Static
        let mut guess = String::new();
        println!("Please input your guess.");

        // & indicates that the argument is a reference. Similar to a pointer?
        // read_line returns an io::Result. Results are enums with variants Ok and Err.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // must convert otherwise we get a typing error during comparison later
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        // match expressions are made up of 'arms'
        // an arm is a pattern and a function
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smol"),
            Ordering::Equal => {
                println!("You're a winner");
                break;
            },
            Ordering::Greater => println!("Yuge"),
        }
    }
}
